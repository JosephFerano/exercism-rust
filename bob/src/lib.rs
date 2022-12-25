fn all_uppercase(phrase :&str) -> bool {
    let mut filtered = phrase.chars().filter(|c| c.is_alphabetic());
    filtered.clone().peekable().peek().is_some() && filtered.all(|c| c.is_uppercase())
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        "" =>  "Fine. Be that way!",
        m if m.chars().last().unwrap() == '?' && all_uppercase(&m) => "Calm down, I know what I'm doing!",
        m if m.chars().last().unwrap() == '?' => "Sure.",
        m if all_uppercase(&m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
