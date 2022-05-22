#![feature(array_windows)]

pub fn build_proverb(list: &[&str]) -> String {
  let mut string = String::new();

  if list.is_empty() {
    return string;
  }

  list.array_windows().for_each(|[a, b]| string.push_str(&format!("For want of a {} the {} was lost.\n", a, b)));
  string.push_str(&format!("And all for the want of a {}.", list[0]));

  string
}
