pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|digit| factors.iter().any(|f| *f != 0 && digit % f == 0))
        .sum()
}
