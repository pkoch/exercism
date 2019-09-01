use std::collections::HashMap;

fn forward() -> Vec<u8> {
    (b'a'..=b'z').chain(b'0'..=b'9').collect()
}

fn backward() -> Vec<u8> {
    (b'a'..=b'z').rev().chain(b'0'..=b'9').collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // I wish I could easily make this static.
    let table: HashMap<u8, u8> = forward().into_iter().zip(backward()).collect();

    plain
        .to_lowercase()
        .bytes()
        .filter_map(|a| table.get(&a))
        .map(|a| *a as char)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|a| a.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode(cipher).replace(' ', "")
}
