pub fn factors(n: u64) -> Vec<u64> {
  let mut factors = Vec::new();
  let mut num = n;

  for factor in (2..).take_while(|i| i * i <= n) {
    while num % factor == 0 {
      factors.push(factor);
      num /= factor;
    }
  }

  if num > 1 {
    factors.push(num);
  }

  factors
}
