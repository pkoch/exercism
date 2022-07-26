use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Rectangle {
    pub lt: Coordinate,
    pub rt: Coordinate,
    pub lb: Coordinate,
    pub rb: Coordinate,
}

impl Rectangle {
    /// ```
    /// # use rectangles::*;
    /// assert!(
    ///     Rectangle{
    ///         lt: Coordinate{x: 0, y: 0},
    ///         rt: Coordinate{x: 1, y: 0},
    ///         lb: Coordinate{x: 0, y: 1},
    ///         rb: Coordinate{x: 1, y: 1},
    ///     }.is_connected(&[
    ///         "++",
    ///         "++",
    ///     ]),
    /// );
    /// assert!(
    ///     Rectangle{
    ///         lt: Coordinate{x: 0, y: 0},
    ///         rt: Coordinate{x: 3, y: 0},
    ///         lb: Coordinate{x: 0, y: 1},
    ///         rb: Coordinate{x: 3, y: 1},
    ///     }.is_connected(&[
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
    ///           rt: Coordinate{x: 2, y: 0},
    ///           lb: Coordinate{x: 0, y: 2},
    ///           rb: Coordinate{x: 2, y: 2},
    ///       }.is_connected(&grid[..]),
    ///       *result,
    ///       "{:?} failed", n,
    ///   );
    /// }
    /// ```
    pub fn is_connected(&self, lines: &[&str]) -> bool {
        [
            ((self.lt.x, self.rt.x), self.lt.y),
            ((self.lb.x, self.rb.x), self.lb.y),
        ]
        .iter()
        .all(|((x_start, x_end), y)| {
            (*x_start..=*x_end).all(|x| ['-', '+'].contains(&char_at(lines, x, *y).unwrap_or(' ')))
        }) && [
            (self.lt.x, (self.lt.y, self.lb.y)),
            (self.rt.x, (self.rt.y, self.rb.y)),
        ]
        .iter()
        .all(|(x, (y_start, y_end))| {
            (*y_start..=*y_end).all(|y| ['|', '+'].contains(&char_at(lines, *x, y).unwrap_or(' ')))
        })
    }
}

pub fn char_at(lines: &[&str], x: usize, y: usize) -> Option<char> {
    lines
        .iter()
        .nth(y)
        .map(|l| l.chars().nth(x).map(|c| c))
        .flatten()
}

pub fn count(lines: &[&str]) -> u32 {
    let crosses = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '+' {
                    Some(Coordinate { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Coordinate>>();

    let mut detected_rectangles = HashSet::<Rectangle>::new();
    for lt in &crosses[..] {
        for rb in &crosses[..] {
            if lt.x == rb.x || lt.y == rb.y {
                continue;
            }

            let rt = &Coordinate { x: rb.x, y: lt.y };
            let lb = &Coordinate { x: lt.x, y: rb.y };

            if Some('+') != char_at(lines, rt.x, rt.y) || Some('+') != char_at(lines, lb.x, lb.y) {
                continue;
            }

            let candidate = Rectangle{lt: *lt, rt: *rt, lb: *lb, rb: *rb};
            if candidate.is_connected(lines) {
                println!("{:?}", candidate);
                detected_rectangles.insert(candidate);
            }
        }
    }

    detected_rectangles.len() as u32
}