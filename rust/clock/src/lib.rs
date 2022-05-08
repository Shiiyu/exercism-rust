use std::fmt::{Display, Formatter, Result};

const HOUR: i32 = 60;
const DAY: i32 = 1440;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
  hours: u8,
  minutes: u8
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let mut total = (hours * HOUR + minutes) % DAY;

    if total < 0 {
      total += DAY;
    }

    Self { hours: (total / HOUR) as u8, minutes: (total % HOUR) as u8 }
  }

  pub fn add_minutes(&self, minutes: i32) -> Self {
    let mut total = (self.hours as i32 * HOUR + self.minutes as i32 + minutes) % DAY;

    if total < 0 {
      total += DAY;
    }

    Self { hours: (total / HOUR) as u8, minutes: (total % HOUR) as u8 }
  }
}

impl Display for Clock {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
  }
}
