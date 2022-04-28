#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Unequal,
  Sublist,
  Superlist
}

pub fn sublist<T: Eq>(first_list: &[T], second_list: &[T]) -> Comparison {
  match (first_list.len(), second_list.len()) {
    (0, 0) => Comparison::Equal,
    (0, _) => Comparison::Sublist,
    (_, 0) => Comparison::Superlist,
    (first, second) if first > second => {
      if first_list.windows(second).any(|w| w == second_list) {
        Comparison::Superlist
      } else {
        Comparison::Unequal
      }
    },
    (first, second) if first < second => {
      if second_list.windows(first).any(|w| w == first_list) {
        Comparison::Sublist
      } else {
        Comparison::Unequal
      }
    },
    (_, _) => {
      if first_list == second_list {
        Comparison::Equal
      } else {
        Comparison::Unequal
      }
    },
  }
}
