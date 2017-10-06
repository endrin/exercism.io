#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

custom_derive! {
    #[derive(Copy, Clone, Debug, PartialEq,
        IterVariants(AllergenVariants))]
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
}

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
        Allergen::iter_variants()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
