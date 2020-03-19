use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut minutes = self.minutes;
        let mut hours = self.hours;

        while minutes >= 60 {
            hours += 1;
            minutes -= 60;
        }

        while minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        while hours >= 24 {
            hours -= 24;
        }

        while hours < 0 {
            hours += 24;
        }

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self
    }
}
