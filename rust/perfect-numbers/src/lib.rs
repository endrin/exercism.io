use std::cmp::Ordering::*;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Abundant,
    Deficient,
    Perfect,
}

pub fn classify(
    num: u64,
) -> Result<Classification, &'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let sum_factors =
        (1..num / 2 + 1).filter(|f| num % f == 0).sum();

    Ok(match num.cmp(&sum_factors) {
        Less => Classification::Abundant,
        Greater => Classification::Deficient,
        Equal => Classification::Perfect,
    })
}
