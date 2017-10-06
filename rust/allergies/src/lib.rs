#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
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

pub struct Allergies {
    code: u8,
}

impl Allergies {
    pub fn new(code: u32) -> Self {
        Allergies {
            code: (code % 256) as u8,
        }
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        let bit = 1 << *a as u8;
        self.code & bit == bit
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ].into_iter()
            .filter_map(|a| if self.is_allergic_to(a) {
                Some(*a)
            } else {
                None
            })
            .collect()
    }
}
