/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let chars = &code.to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    match &chars[..] {
        [] => false,
        [_] => false,
        chars => {
            let mut sum = 0;
            for (i, &c) in chars.iter().rev().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    if i % 2 == 0 {
                        sum += digit
                    } else {
                        let mut doubled = digit * 2;
                        if doubled > 9 { doubled -= 9 }
                        sum += doubled
                    }
                } else {
                    return false
                }
            }
            sum % 10 == 0
        }
    }
}
