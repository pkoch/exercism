use std::collections::HashMap;

fn forward() -> Vec<char> {
    (b'a'..=b'z')
        .chain(b'0'..=b'9')
        .map(|a| a as char)
        .collect()
}

fn backward() -> Vec<char> {
    (b'a'..=b'z')
        .rev()
        .chain(b'0'..=b'9')
        .map(|a| a as char)
        .collect()
}

fn build_map(from: Vec<char>, to: Vec<char>) -> HashMap<char, char> {
    from.into_iter().zip(to).collect()
}

trait Trans {
    fn trans(&self, s: &str) -> String;
}

impl Trans for HashMap<char, char> {
    fn trans(&self, s: &str) -> String {
        s.to_lowercase()
            .chars()
            .filter_map(|a| self.get(&a))
            .collect()
    }
}

trait GroupChars {
    fn group_chars(&self, n: usize, separator: &str) -> String;
}

impl GroupChars for String {
    fn group_chars(&self, n: usize, separator: &str) -> String {
        self.chars()
            .collect::<Vec<_>>()
            .chunks(n)
            .map(|a| a.iter().collect())
            .collect::<Vec<String>>()
            .join(separator)
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    build_map(forward(), backward())
        .trans(plain)
        .group_chars(5, " ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    build_map(backward(), forward())
        .trans(cipher)
}
