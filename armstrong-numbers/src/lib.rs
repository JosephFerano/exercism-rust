pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let amount: u32 = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0).pow(digits.len() as u32))
        .sum();
    amount == num
}
