pub fn nth(n: usize) -> u32 {
  let mut primes = vec![2, 3];
  let mut candidate = 4;

  while n + 1 > primes.len() {
    if candidate % 2 == 0 || candidate % 3 == 0 {
      candidate += 1;

      continue;
    }

    let mut multiple = 6;

    loop {
      if candidate % (multiple - 1) == 0 || candidate % (multiple + 1) == 0 {
        if !primes.iter().any(|i| candidate % i == 0) {
          primes.push(candidate);
        }

        candidate += 1;

        break;
      }

      if multiple * multiple > candidate {
        primes.push(candidate);
        candidate += 1;

        break;
      }

      multiple += 6;
    }
  }

  primes[n]
}
