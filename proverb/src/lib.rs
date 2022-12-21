pub fn build_proverb(list: &[&str]) -> String {
    let mut string = String::new();
    match list.iter().next() {
        Some(head) => {
            for entry in list.windows(2) {
                let line = format!("For want of a {} the {} was lost.\n", entry[0], entry[1]);
                string.push_str(&line);
            }
            let last_line = format!("And all for the want of a {head}.");
            string + &last_line
        }
        None => string,
    }
}
