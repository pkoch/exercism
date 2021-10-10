#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref FORWARD: Vec<char> = (b'a'..=b'z')
        .chain(b'0'..=b'9')
        .map(|a| a as char)
        .collect();
    static ref BACKWARD: Vec<char> = (b'a'..=b'z')
        .rev()
        .chain(b'0'..=b'9')
        .map(|a| a as char)
        .collect();
    static ref TABLE: HashMap<&'static char, &'static char> =
        FORWARD.iter().zip(BACKWARD.iter()).collect();
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
    s.chars()
        .filter_map(|c| {
            TABLE.get(&c.to_ascii_lowercase()).cloned()
        })
        .collect::<String>()
}
