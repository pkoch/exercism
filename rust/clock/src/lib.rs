use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Rem};

trait DivMod: Sized + Add + Div + Rem {
    fn div_mod(self, other: Self) -> (Self, Self);
}

impl DivMod for i32 {
    fn div_mod(self, other: Self) -> (Self, Self) {
        (self / other, ((self % other) + other) % other)
    }
}

static MINUTES_IN_HOUR: i32 = 60;
static MINUTES_IN_DAY: i32 = 24 * MINUTES_IN_HOUR;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: 0 }
            .add_minutes(hours * MINUTES_IN_HOUR)
            .add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: (self.minutes + minutes).div_mod(MINUTES_IN_DAY).1,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (hours, minutes) = self.minutes.div_mod(MINUTES_IN_HOUR);

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
