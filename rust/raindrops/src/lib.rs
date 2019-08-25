pub fn sounder<'a>(div: u32, msg: &'a str) -> impl Fn(u32) -> Option<&'a str>
{
    move |n: u32|{
        if n % div != 0 {
            return None;
        };

        Some(msg)
    }
}

pub trait NonEmptyOr {
    fn non_empty_or(self: Self, alternative: Self) -> Self;
}

impl NonEmptyOr for String {
    fn non_empty_or(self: Self, alternative: Self) -> Self{
        if !self.is_empty() {
            return self;
        };

        alternative
    }
}

pub fn raindrops(n: u32) -> String {
    [
        sounder(3, "Pling"),
        sounder(5, "Plang"),
        sounder(7, "Plong"),
    ]
        .iter()
        .filter_map(|f| f(n))
        .collect::<Vec<&str>>()
        .concat()
        .non_empty_or(n.to_string())
}
