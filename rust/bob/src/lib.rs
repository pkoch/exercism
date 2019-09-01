pub fn reply(message: &str) -> &str {
    let s = message.trim();

    if s.is_empty() {
        return "Fine. Be that way!"
    }

    let is_all_caps = s.chars().any(char::is_alphabetic) && s.to_uppercase() == s;
    let is_question = s.ends_with("?");
    match (is_question, is_all_caps) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        (_, _) => "Whatever.",
    }
}
