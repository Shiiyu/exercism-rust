pub fn series(digits: &str, len: usize) -> Vec<String> {
  if len > digits.len() {
    return vec![];
  } else if len == 0 {
    return vec![String::new(); digits.len() + 1];
  }

  digits.chars().collect::<Vec<char>>().windows(len).map(|w| w.iter().collect()).collect()
}
