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

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let __ = plain;
    let __ = atbash_str(__);
    let __ = chunk_str(&__, 5);
    let __ = __.join(" ");
    __
}

fn chunk_str(s: &String, n: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|a| a.iter().collect())
        .collect::<Vec<String>>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode(cipher).replace(' ', "")
}

pub fn atbash_str(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter_map(atbash_char)
        .collect::<String>()
}

pub fn atbash_char(c: char) -> Option<char> {
    // I wish I could easily make this static.
    let table: HashMap<char, char> = forward().into_iter().zip(backward()).collect();

    table.get(&c).map(|a| *a)
}
