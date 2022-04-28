pub struct Allergies(u8);

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
  fn from(val: u8) -> Self {
    match val {
      1 => Allergen::Eggs,
      2 => Allergen::Peanuts,
      4 => Allergen::Shellfish,
      8 => Allergen::Strawberries,
      16 => Allergen::Tomatoes,
      32 => Allergen::Chocolate,
      64 => Allergen::Pollen,
      128 => Allergen::Cats,
      _ => panic!("Invalid Allergen")
    }
  }
}

impl Allergies {
  pub fn new(score: u32) -> Self {
    Self(score as u8)
  }

  pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
    self.0 & *allergen as u8 == *allergen as u8
  }

  pub fn allergies(&self) -> Vec<Allergen> {
    let mut allergies = Vec::new();

    (0..8).into_iter().for_each(|i| {
      if self.0 & (1 << i) > 0 {
        allergies.push(Allergen::from(1 << i))
      }
    });

    allergies
  }
}
