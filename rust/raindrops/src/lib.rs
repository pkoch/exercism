pub fn divmsg(div: u32, msg: &str, n: u32) -> String {
    if n % div != 0 {
        return String::new();
    }

    return String::from(msg);
}

pub fn raindrops(n: u32) -> String {
    let msg = [
        divmsg(3, "Pling", n),
        divmsg(5, "Plang", n),
        divmsg(7, "Plong", n),
    ]
    .join("");

    if !msg.is_empty() {
        return msg;
    };

    n.to_string()
}
