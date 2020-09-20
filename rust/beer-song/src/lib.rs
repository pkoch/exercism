pub fn verse(n: i32) -> String {
    match n {
        0 => [
            "No more bottles of beer on the wall, no more bottles of beer.\n".into(),
            "Go to the store and buy some more, 99 bottles of beer on the wall.\n".into(),
        ],
        1 => [
            "1 bottle of beer on the wall, 1 bottle of beer.\n".into(),
            "Take it down and pass it around, no more bottles of beer on the wall.\n".into(),
        ],
        2 => [
            "2 bottles of beer on the wall, 2 bottles of beer.\n".into(),
            "Take one down and pass it around, 1 bottle of beer on the wall.\n".into(),
        ],
        n => [
            format!("{0} bottles of beer on the wall, {0} bottles of beer.\n", n),
            format!(
                "Take one down and pass it around, {} bottles of beer on the wall.\n",
                n - 1
            ),
        ],
    }
    .join("")
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
