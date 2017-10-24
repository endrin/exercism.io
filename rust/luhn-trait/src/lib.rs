// Most of the actual code is actually moved
// from my other Luhn's

#![feature(specialization)]

use std::fmt;

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

default impl<T> Luhn for T
where
    T: fmt::Display,
{
    fn clean(&self) -> Option<Vec<usize>> {
        let input: String = format!("{}", self);

        if input.chars().any(|c| !"0123546789 ".contains(c))
        {
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
}

impl<T> Luhn for T
where
    T: fmt::Display + Copy + Into<u64>,
{
    fn clean(&self) -> Option<Vec<usize>> {
        let mut input = (*self).into();

        let mut ds: Vec<usize> = vec![];

        while input > 0 {
            ds.push((input % 10u64) as usize);
            input /= 10;
        }

        if ds.len() >= 2 {
            Some(ds)
        } else {
            None
        }
    }
}
