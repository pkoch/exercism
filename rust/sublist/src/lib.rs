use std::cmp::min;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn first_positions<T: PartialOrd>(l: &[T], target: &T) -> Vec<usize> {
    l.iter()
        .enumerate()
        .filter(|(_i, v)| *v == target)
        .map(|(i, _v)| i)
        .collect()
}

pub fn shared_prefix_len<T: PartialOrd>(la: &[T], lb: &[T]) -> usize {
    let mut it = la
        .iter()
        .zip(lb)
        .enumerate()
        .skip_while(|(_i, (a, b))| *a == *b);

    match it.next() {
        Some((i, _)) => i - 1,
        None => min(la.len(), lb.len()),
    }
}

pub fn is_substreak_of<T: PartialOrd>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 {
        return true;
    }
    if b.len() == 0 {
        return false;
    }
    let &first_a = &a.get(0).unwrap();
    let len_a = a.len();

    first_positions(b, first_a)
        .iter()
        .map(|n| &b[*n..])
        .any(|bi| shared_prefix_len(a, bi) == len_a)
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
