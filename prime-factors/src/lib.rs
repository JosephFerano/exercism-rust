pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut divisor = 2;
    let mut curr = n;
    while divisor <= curr {
        if curr % divisor == 0 {
            factors.push(divisor);
            curr /= divisor;
        } else {
            divisor += 1;
        }
    }
    factors
}
