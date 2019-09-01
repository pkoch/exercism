pub fn reply(message: &str) -> &str {
    let mut s = message.to_string();
    s.retain(|c| !c.is_whitespace());

    if s.is_empty() {
        return "Fine. Be that way!"
    }

    let is_all_caps = s.to_uppercase() == s && s.to_lowercase() != s.to_uppercase();
    let is_question = s.ends_with("?");
    match (is_question, is_all_caps) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        (_, _) => "Whatever.",
    }
}
