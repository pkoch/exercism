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
    let mut result = vec![vec![32; line_len]; line_count];

    let lines = &minefield.iter().map(|l| l.as_bytes()).collect::<Vec<_>>()[..];
    for (y, line) in lines.iter().enumerate() {
        for (x, pos) in line.iter().enumerate() {
            if *pos == ('*' as u8) {
                result[y][x] = *pos;
                continue;
            }

            let mine_count = sorrounding_coordinates(x, y, line_len, line_count)
                .iter()
                .filter(|(x, y)| lines[*y][*x] == ('*' as u8))
                .count();
            let new_symbol = if mine_count == 0 {
                ' '
            } else {
                mine_count.to_string().chars().next().unwrap()
            };

            result[y][x] = new_symbol as u8;
        }
    }
    result
        .iter()
        .map(|line| String::from_utf8_lossy(line).to_string())
        .collect()
}
