pub fn is_armstrong_number(num: u32) -> bool {
  let mut number = num;
  let mut digits = Vec::new();

  while number > 0 {
    digits.push(number % 10);

    number /= 10;
  }

  num == digits.iter().map(|d| d.pow(digits.len() as u32)).sum()
}
