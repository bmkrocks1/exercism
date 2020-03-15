pub fn is_prime(x: u32) -> bool {
  !(2..x).any(|div| x % div == 0)
}

pub fn next_odd_number(n: u32) -> u32 {
  n % 2 + 1 + n
}

pub fn nth(n: u32) -> u32 {
  let mut prime_index = 0;
  let mut last_computed_prime = 2;
  let mut odd_number = next_odd_number(last_computed_prime);

  while n > prime_index {
    if is_prime(odd_number) {
      last_computed_prime = odd_number;
      prime_index += 1;
      odd_number = next_odd_number(last_computed_prime);
    } else {
      odd_number = next_odd_number(odd_number);
    }
  }

  last_computed_prime
}
