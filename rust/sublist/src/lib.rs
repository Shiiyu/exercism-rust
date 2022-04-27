#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
  if first_list.is_empty() && second_list.is_empty() {
    return Comparison::Equal;
  } else if first_list.is_empty() {
    return Comparison::Sublist;
  } else if second_list.is_empty() {
    return Comparison::Superlist;
  }

  if first_list.len() < second_list.len() {
    let iter = second_list.windows(first_list.len());

    for window in iter {
      if first_list == window {
        return Comparison::Sublist;
      }
    }

    return Comparison::Unequal;
  } else if first_list.len() > second_list.len() {
    let iter = first_list.windows(second_list.len());

    for window in iter {
      if second_list == window {
        return Comparison::Superlist;
      }
    }

    return Comparison::Unequal;
  } else {
    if first_list == second_list {
      return Comparison::Equal;
    }

    return Comparison::Unequal;
  }
}
