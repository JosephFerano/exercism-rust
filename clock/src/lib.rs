use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mins = self.minutes % 60;
        let hours = (self.minutes / 60) % 24;
        write!(f, "{:#02}:{:#02}", hours, mins)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{ minutes: (1440 + ((hours * 60 + minutes) % 1440)) % 1440 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes as i32 + minutes)
    }
}
