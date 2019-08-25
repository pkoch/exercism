pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    };

    let mut res = list
        .windows(2)
        .map(|pair| match pair {
            [a, b] => format!("For want of a {} the {} was lost.", a, b),
            other => panic!("Wierd window! `{:?}`", other),
        })
        .collect::<Vec<String>>();

    res.push(format!("And all for the want of a {}.", list[0]));
    res.join("\n")
}
