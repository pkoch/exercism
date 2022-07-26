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
    ///     }.exists_in_grid(Grid(&[
    ///         "++",
    ///         "++",
    ///     ])),
    /// );
    /// assert!(
    ///     Rectangle{
    ///         lt: Coordinate{x: 0, y: 0},
    ///         rb: Coordinate{x: 3, y: 1},
    ///     }.exists_in_grid(Grid(&[
    ///         "+--+",
    ///         "+--+",
    ///     ])),
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
    ///       }.exists_in_grid(Grid(&grid[..])),
    ///       *result,
    ///       "{:?} failed", n,
    ///   );
    /// }
    /// ```
    pub fn exists_in_grid(&self, grid: Grid) -> bool {
        Some('+') == grid.char_at(self.r(), self.t())
            && Some('+') == grid.char_at(self.l(), self.b())
            && [
                ((self.l(), self.r()), self.t()),
                ((self.l(), self.r()), self.b()),
            ]
            .iter()
            .all(|((x_start, x_end), y)| {
                (*x_start..=*x_end)
                    .all(|x| ['-', '+'].contains(&grid.char_at(x, *y).unwrap_or(' ')))
            })
            && [
                (self.l(), (self.t(), self.b())),
                (self.r(), (self.t(), self.b())),
            ]
            .iter()
            .all(|(x, (y_start, y_end))| {
                (*y_start..=*y_end)
                    .all(|y| ['|', '+'].contains(&grid.char_at(*x, y).unwrap_or(' ')))
            })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Grid<'a, 'b>(pub &'a [&'b str]);

impl<'a, 'b> Grid<'a, 'b> {
    pub fn char_at(&self, x: usize, y: usize) -> Option<char> {
        self.0.get(y).and_then(|l| l.chars().nth(x))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coordinate, char)> + 'a {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Coordinate { x, y }, c))
            })
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let grid = Grid(lines);

    let crosses: Vec<Coordinate> = grid
        .iter()
        .filter(|(_coord, char)| *char == '+')
        .map(|(coord, _char)| coord)
        .collect();

    crosses
        .iter()
        .flat_map(|lt| crosses.iter().map(move |rb| (lt, rb)))
        .filter(|(lt, rb)| rb.x > lt.x && rb.y > lt.y)
        .map(|(lt, rb)| Rectangle { lt: *lt, rb: *rb })
        .filter(|r| r.exists_in_grid(grid))
        .count() as u32
}
