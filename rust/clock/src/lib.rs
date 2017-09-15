use std::fmt;

// Amount of minutes in a day, 24 * 60
static DAY_MINS: i32 = 1440;

#[derive(Debug)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        // Total amount of entered minutes
        let time = hours * 60 + minutes;

        // The idea here is to work around negative modulo
        // If it negative, it will just be substracted
        // from max value.
        // It it is positive, it will be divided twice,
        // but lets hope that's not a big performance loss
        let wrapped_mins =
            (time % DAY_MINS + DAY_MINS) % DAY_MINS;

        Clock(wrapped_mins)
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        // All wrapping is done in ::new method, so
        // clock with extra minutes is passed there
        Clock::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Clock(time) = *self;
        write!(f, "{:02}:{:02}", time / 60, time % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.0 == other.0
    }
}
