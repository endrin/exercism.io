// There is no such thing as overkill

use std::ops::Rem;

trait Divisible
where
    Self: Copy + Rem<Output = Self> + Eq + Default,
{
    fn is_divisible_by(&self, n: Self) -> bool {
        *self % n == Default::default()
    }
}

impl Divisible for u32 {}

pub fn is_leap_year(year: u32) -> bool {
    (year.is_divisible_by(4) && !year.is_divisible_by(100)) || year.is_divisible_by(400)
}
