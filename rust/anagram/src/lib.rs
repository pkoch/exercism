use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn sorted_chars(s: &str) -> String {
    let mut v = s.graphemes(true)
        .collect::<Vec<_>>();
    v.sort();
    v.join("")
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();

    possible_anagrams
        .iter()
        .filter(|s| {
            let sl = s.to_lowercase();
            sl != lower_word && sorted_chars(&sl) == sorted_chars(&lower_word)
        })
        .map(|a| *a)
        .collect::<HashSet<&str>>()
}
