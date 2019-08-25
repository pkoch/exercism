use std::iter::once;

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

pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.", pair[0], pair[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<String>>()
        .join("\n")
        .non_empty_or(String::new())
}
