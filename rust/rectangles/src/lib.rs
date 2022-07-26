#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Rectangle {
    pub lt: Coordinate,
    pub rb: Coordinate,
}

impl Rectangle {
    pub fn l(&self) -> usize {
        self.lt.x
    }
    pub fn r(&self) -> usize {
        self.rb.x
    }
    pub fn t(&self) -> usize {
        self.lt.y
    }
    pub fn b(&self) -> usize {
        self.rb.y
    }
    /// ```
    /// # use rectangles::*;
    /// assert!(
    ///     Rectangle{
    ///         lt: Coordinate{x: 0, y: 0},
    ///         rb: Coordinate{x: 1, y: 1},
    ///     }.exists_in_grid(&[
    ///         "++",
    ///         "++",
    ///     ]),
    /// );
    /// assert!(
    ///     Rectangle{
    ///         lt: Coordinate{x: 0, y: 0},
    ///         rb: Coordinate{x: 3, y: 1},
    ///     }.exists_in_grid(&[
    ///         "+--+",
    ///         "+--+",
    ///     ]),
    /// );
    /// for (n, (result, grid)) in [
    ///     (true, &[
    ///       "+-+",
    ///       "| |",
    ///       "+-+",
    ///     ]),
    ///     (true, &[
    ///       "+++",
    ///       "+ +",
    ///       "+++",
    ///     ]),
    ///     (false, &[
    ///       "+|+",
    ///       "- -",
    ///       "+|+",
    ///     ]),
    ///     (false, &[
    ///       "+-+",
    ///       "| |",
    ///       " -+",
    ///     ]),
    ///     (false, &[
    ///       "+- ",
    ///       "| |",
    ///       "+-+",
    ///     ]),
    ///     (false, &[
    ///       "+-+",
    ///       "|  ",
    ///       "+-+",
    ///     ]),
    ///     (false, &[
    ///       "+-+",
    ///       "  |",
    ///       "+-+",
    ///     ]),
    ///     (false, &[
    ///       "+ +",
    ///       "| |",
    ///       "+-+",
    ///     ]),
    ///     (false, &[
    ///       "+-+",
    ///       "| |",
    ///       "+ +",
    ///     ]),
    /// ].iter().enumerate() {
    ///   assert_eq!(
    ///       Rectangle{
    ///           lt: Coordinate{x: 0, y: 0},
    ///           rb: Coordinate{x: 2, y: 2},
    ///       }.exists_in_grid(&grid[..]),
    ///       *result,
    ///       "{:?} failed", n,
    ///   );
    /// }
    /// ```
    pub fn exists_in_grid(&self, lines: &[&str]) -> bool {
        Some('+') == char_at(lines, self.r(), self.t())
            && Some('+') == char_at(lines, self.l(), self.b())
            && [
                ((self.l(), self.r()), self.t()),
                ((self.l(), self.r()), self.b()),
            ]
            .iter()
            .all(|((x_start, x_end), y)| {
                (*x_start..=*x_end)
                    .all(|x| ['-', '+'].contains(&char_at(lines, x, *y).unwrap_or(' ')))
            })
            && [
                (self.l(), (self.t(), self.b())),
                (self.r(), (self.t(), self.b())),
            ]
            .iter()
            .all(|(x, (y_start, y_end))| {
                (*y_start..=*y_end)
                    .all(|y| ['|', '+'].contains(&char_at(lines, *x, y).unwrap_or(' ')))
            })
    }
}

pub fn char_at(lines: &[&str], x: usize, y: usize) -> Option<char> {
    lines
        .get(y)
        .and_then(|l| l.chars().nth(x))
}

fn when_then<V, F: Fn() -> V>(when: bool, then: F) -> Option<V> {
    if when {
        Some(then())
    } else {
        None
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let crosses = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| when_then(c == '+', || Coordinate { x, y }))
        })
        .collect::<Vec<Coordinate>>();

    crosses
        .iter()
        .flat_map(|lt| crosses.iter().map(move |rb| (lt, rb)))
        .filter_map(|(lt, rb)| {
            when_then(rb.x > lt.x && rb.y > lt.y, || Rectangle {
                lt: *lt,
                rb: *rb,
            })
        })
        .filter(|r| r.exists_in_grid(lines))
        .count() as u32
}
