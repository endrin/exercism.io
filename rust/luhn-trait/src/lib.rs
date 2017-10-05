// Most of the actual code is actually moved
// from my other Luhn's

extern crate num;

use num::{Integer, NumCast};

const NORMAL: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const DOUBLED: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
static TRANSFORMATIONS: [[u32; 10]; 2] = [NORMAL, DOUBLED];

pub trait Luhn {
    fn clean(&self) -> Option<Vec<usize>>;

    fn valid_luhn(&self) -> bool {
        self.clean()
            .map(|digits| {
                let transformations =
                    TRANSFORMATIONS.iter().cycle();

                digits
                    .into_iter()
                    .zip(transformations)
                    .map(|(c, transform)| transform[c])
                    .sum::<u32>() % 10 == 0
            })
            .unwrap_or(false)
    }
}

impl<'a> Luhn for &'a str {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_string(&self)
    }
}

impl Luhn for String {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_string(&self)
    }
}

impl Luhn for u8 {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_integer(self)
    }
}

impl Luhn for u16 {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_integer(self)
    }
}

impl Luhn for u32 {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_integer(self)
    }
}

impl Luhn for u64 {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_integer(self)
    }
}

impl Luhn for usize {
    fn clean(&self) -> Option<Vec<usize>> {
        clean_integer(self)
    }
}

fn clean_string<'a, T>(input: &'a T) -> Option<Vec<usize>>
where
    T: ToString,
{
    let input: String = input.to_string();

    if input.chars().any(|c| !"0123546789 ".contains(c)) {
        return None;
    }

    let result: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .rev()
        .collect();

    if result.len() >= 2 {
        Some(result)
    } else {
        None
    }
}

fn clean_integer<T>(input: &T) -> Option<Vec<usize>>
where
    T: Copy + Integer + NumCast,
{
    let mut ds = vec![];
    let mut input: T = input.clone();

    while input > num::zero() {
        ds.push(
            input
                .mod_floor(&T::from(10usize).unwrap())
                .to_usize()
                .unwrap(),
        );
        input = input.div_floor(&T::from(10usize).unwrap());
        // Around here I started thinking that it was
        // better to just convert all types to string
    }

    if ds.len() >= 2 {
        Some(ds)
    } else {
        None
    }
}
