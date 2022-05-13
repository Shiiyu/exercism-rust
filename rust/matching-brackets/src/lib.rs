fn equals(a: char, b: char) -> bool {
  match a {
    '(' => b == ')',
    '[' => b == ']',
    '{' => b == '}',
    _ => unreachable!()
  }
}

pub fn brackets_are_balanced(string: &str) -> bool {
  let mut openings = Vec::new();

  for bracket in string.chars() {
    if matches!(bracket, '(' | '[' | '{') {
      openings.push(bracket);
    } else if !openings.is_empty() && equals(openings[openings.len() - 1], bracket) {
      openings.pop();
    } else if matches!(bracket, ')' | ']' | '}') {
      return false;
    }
  }

  if !openings.is_empty() {
    return false;
  }

  true
}
