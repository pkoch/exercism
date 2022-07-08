use std::collections::HashMap;
use std::thread;

pub fn char_count(chunk: Vec<String>) -> HashMap<char, usize> {
    chunk
        .iter()
        .flat_map(|line| line.chars())
        .fold(HashMap::new(), |mut a, c| {
            *a.entry(c).or_default() += 1;
            a
        })
}

pub fn merge_counts(mut a: HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    b.iter().for_each(|(k, v)| {
        *a.entry(*k).or_default() += v;
    });
    a
}

pub fn into_clean_sting(s: &str) -> String {
    let mut s_ = s.to_lowercase();
    s_.retain(|c| !r#"(),".;:'1234567890"#.contains(c));
    s_
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input
        .chunks(input.len() / worker_count + 1)
        .map(|chunk| {
            let v = chunk
                .iter()
                .map(|s| into_clean_sting(s))
                .collect::<Vec<_>>();
            thread::spawn(move || char_count(v))
        })
        .fold(HashMap::<char, usize>::new(), |a, h| {
            merge_counts(a, &h.join().unwrap())
        })
}
