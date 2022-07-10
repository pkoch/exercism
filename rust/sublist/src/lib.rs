#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_substreak_of<T: PartialOrd>(a: &[T], b: &[T]) -> bool {
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 {
        return true;
    }
    if len_b == 0 {
        return false;
    }

    b.windows(len_a).any(|candidate| a == candidate)
}

pub fn sublist<T: PartialOrd>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (
        is_substreak_of(first_list, second_list),
        is_substreak_of(second_list, first_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
