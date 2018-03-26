#![allow(non_upper_case_globals)]

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Allergen: u32 {
        const Eggs          = 0b0000_0001;
        const Peanuts       = 0b0000_0010;
        const Shellfish     = 0b0000_0100;
        const Strawberries  = 0b0000_1000;
        const Tomatoes      = 0b0001_0000;
        const Chocolate     = 0b0010_0000;
        const Pollen        = 0b0100_0000;
        const Cats          = 0b1000_0000;
    }
}

pub struct Allergies(Allergen);

impl Allergies {
    pub fn new(code: u32) -> Self {
        Allergies(Allergen::from_bits_truncate(code))
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        self.0.contains(*a)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .into_iter()
            .map(|bit| {
                Allergen::from_bits_truncate(1 << bit)
            })
            .filter_map(|flag| {
                if self.is_allergic_to(&flag) {
                    Some(flag)
                } else {
                    None
                }
            })
            .collect()
    }
}
