pub fn sorrounding_coordinates(
    x: usize,
    y: usize,
    line_len: usize,
    line_count: usize,
) -> Vec<(usize, usize)> {
    let x_: i64 = x.try_into().unwrap();
    let y_: i64 = y.try_into().unwrap();
    let line_len_ = line_len.try_into().unwrap();
    let line_count_ = line_count.try_into().unwrap();

    let positions: &[(i64, i64)] = &[
        (x_ - 1, y_ - 1),
        (x_, y_ - 1),
        (x_ + 1, y_ - 1),
        (x_ - 1, y_),
        //skip x_, y_
        (x_ + 1, y_),
        (x_ - 1, y_ + 1),
        (x_, y_ + 1),
        (x_ + 1, y_ + 1),
    ][..];
    positions
        .iter()
        .filter(|(x, y)| 0 <= *x && *x < line_len_ && 0 <= *y && *y < line_count_)
        .map(|(x, y)| (usize::try_from(*x).unwrap(), usize::try_from(*y).unwrap()))
        .collect()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }

    let line_count = minefield.len();
    let line_len = minefield.get(0).unwrap().len();
    let mut result = vec![vec![' '; line_len]; line_count];

    for (y, line) in minefield.iter().enumerate() {
        for (x, pos) in line.as_bytes().iter().enumerate() {
            result[y][x] = if *pos == ('*' as u8) {
                '*'
            }else{
                let mine_count: u8 = sorrounding_coordinates(x, y, line_len, line_count)
                    .iter()
                    .filter(|(x, y)| minefield[*y].as_bytes()[*x] == ('*' as u8))
                    .count().try_into().unwrap();

                match mine_count {
                    0 => ' ',
                    n => (('0' as u8) + n) as char
                }
            };
        }
    }
    result
        .iter()
        .map(|line| line.into_iter().collect())
        .collect()
}
