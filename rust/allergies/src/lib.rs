use crate::Allergen::*;

pub struct Allergies(Vec<Allergen>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
  Eggs = 1,
  Peanuts = 2,
  Shellfish = 4,
  Strawberries = 8,
  Tomatoes = 16,
  Chocolate = 32,
  Pollen = 64,
  Cats = 128
}

impl Allergen {
  const ALLERGENS: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];
}

impl Allergies {
  pub fn new(score: u32) -> Self {
    Self(Allergen::ALLERGENS.iter().copied().filter(|allergen| (score & *allergen as u32) != 0).collect())
  }

  pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
    self.0.contains(allergen)
  }

  pub fn allergies(&self) -> Vec<Allergen> {
    self.0.clone()
  }
}
