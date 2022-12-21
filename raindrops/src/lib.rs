pub fn raindrops(n: u32) -> String {
    let mut sounds = String::new();
    if n % 3 == 0 { sounds.push_str("Pling") }
    if n % 5 == 0 { sounds.push_str("Plang") }
    if n % 7 == 0 { sounds.push_str("Plong") }
    match sounds.is_empty() {
        true => n.to_string(),
        false => sounds,
    }
}
