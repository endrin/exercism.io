pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const YEAR_DURATION: f64;

    fn years_during(d: &Duration) -> f64 {
        let &Duration(s) = d;
        (s as f64) / Self::YEAR_DURATION
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 0.2408467;
}
impl Planet for Venus {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 0.61519726;
}
impl Planet for Earth {
    const YEAR_DURATION: f64 = 31_557_600_f64;
}
impl Planet for Mars {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 1.8808158;
}
impl Planet for Jupiter {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 11.862615;
}
impl Planet for Saturn {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 29.447498;
}
impl Planet for Uranus {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 84.016846;
}
impl Planet for Neptune {
    const YEAR_DURATION: f64 =
        Earth::YEAR_DURATION * 164.79132;
}
