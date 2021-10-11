use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn sorted_chars(s: &str) -> String {
    let mut v = s.graphemes(true).collect::<Vec<_>>();
    v.sort_unstable();
    v.join("")
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted_lower_word = sorted_chars(&lower_word);

    possible_anagrams
        .iter()
        .filter(|o| {
            let ol = o.to_lowercase();
            ol != lower_word && sorted_chars(&ol) == sorted_lower_word
        })
        .cloned()
        .collect()
}
