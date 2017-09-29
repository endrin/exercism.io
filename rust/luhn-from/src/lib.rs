const NORMAL: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const DOUBLED: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
static TRANSFORMATIONS: [[u32; 10]; 2] = [NORMAL, DOUBLED];

pub struct Luhn {
    digits: Option<Vec<usize>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.digits
            .as_ref()
            .map(|digits| {
                let transformations =
                    TRANSFORMATIONS.iter().cycle();

                digits
                    .iter()
                    .zip(transformations)
                    .map(|(&c, transform)| transform[c])
                    .sum::<u32>() % 10 == 0
            })
            .unwrap_or(false)
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        if input.chars().any(|c| !"0123546789 ".contains(c))
        {
            return Luhn { digits: None };
        }

        let result: Vec<usize> = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .rev()
            .collect();

        Luhn {
            digits: if result.len() >= 2 {
                Some(result)
            } else {
                None
            },
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}

macro_rules! impl_from(
    ($($t:ty),+) => {$(
        impl From<$t> for Luhn
        {
            fn from(input: $t) -> Self {
                let mut ds = vec![];
                let mut input = input;

                while input > 0 {
                    ds.push((input % 10) as usize);
                    input /= 10;
                }

                Luhn {
                    digits: if ds.len() >= 2 {
                        Some(ds)
                    } else {
                        None
                    },
                }
            }
        }
    )+}
);

impl_from!(u8, u16, u32, u64, usize);
