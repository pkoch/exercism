use std::collections::HashSet;

pub struct Allergies{
    allergens: HashSet<Allergen>
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 0b1,
    Peanuts = 0b10,
    Shellfish = 0b100,
    Strawberries = 0b1000,
    Tomatoes = 0b10_000,
    Chocolate = 0b10_0000,
    Pollen = 0b100_0000,
    Cats = 0b1000_0000,
}

static ALL_ALLERGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{
            allergens: ALL_ALLERGIES
                .iter()
                .filter(|a| (**a as u32) & score != 0)
                .map(|a| *a)
                .collect()
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.iter().map(|a| *a).collect()
    }
}
