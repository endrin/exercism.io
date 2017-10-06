#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

use Allergen::*;

lazy_static! {
    static ref ALLERGY_FLAGS: BTreeMap<Allergen, usize> = {
        [(Eggs, 0),
        (Peanuts, 1),
        (Shellfish, 2),
        (Strawberries, 3),
        (Tomatoes, 4),
        (Chocolate, 5),
        (Pollen, 6),
        (Cats, 7)].iter().cloned().collect()
    };
}

pub struct Allergies {
    flags: Vec<u8>,
}

impl Allergies {
    pub fn new(code: u32) -> Self {
        Allergies {
            flags: {
                let mut flags = Vec::with_capacity(8);
                let mut code = code % 256;
                while code > 0 {
                    flags.push((code % 2) as u8);
                    code /= 2;
                }
                flags.resize(8, 0);
                flags
            },
        }
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        self.flags[ALLERGY_FLAGS[a]] == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGY_FLAGS
            .iter()
            .filter_map(
                |(a, _)| if self.is_allergic_to(&a) {
                    Some(a.clone())
                } else {
                    None
                },
            )
            .collect()
    }
}
