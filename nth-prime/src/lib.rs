pub fn nth(n: u32) -> u32 {
    let mut candidate = 2;
    let mut primes = vec![];
    while primes.len() <= n as usize {
        let mut not_prime = false;
        for prime in primes.iter() {
            if candidate % prime == 0 {
                not_prime = true;
                break;
            }
        }
        if !not_prime {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes[n as usize]
}
