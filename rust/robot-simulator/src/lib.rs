// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

pub struct Robot{
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { pos: (x, y), dir: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self{
            dir: match self.dir {
                North => East,
                South => West,
                East => South,
                West => North,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self{
            dir: match self.dir {
                North => West,
                South => East,
                East => North,
                West => South,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = self.pos;
        Self{
            pos: match self.dir {
                North => (x, y + 1),
                South => (x, y - 1),
                East => (x + 1, y),
                West => (x - 1, y),
            },
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |s, i| match i {
            'A' => s.advance(),
            'L' => s.turn_left(),
            'R' => s.turn_right(),
            _ => panic!("Unrecognized instruction: {:?}", i)
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
