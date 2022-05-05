pub fn build_proverb(list: &[&str]) -> String {
  let mut string = String::new();

  if list.is_empty() {
    return string;
  }

  list
    .windows(2)
    .for_each(|words| string.push_str(&format!("For want of a {} the {} was lost.\n", words[0], words[1])));
  string.push_str(&format!("And all for the want of a {}.", list[0]));

  string
}
