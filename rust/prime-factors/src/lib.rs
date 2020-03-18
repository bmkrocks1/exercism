fn is_prime(n: u64) -> bool {
    !(2..n).any(|d| n % d == 0)
}

pub fn factors(limit: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    let mut q: u64 = limit;
    let mut primes = (2..=limit).filter(|n| is_prime(*n));
    let mut current_prime = primes.next();

    while let Some(p) = current_prime {
        if q <= 1 {
            break;
        }

        if q % p == 0 {
            output.push(p);
            q /= p;
        } else {
            current_prime = primes.next();
        }
    }

    output
}
