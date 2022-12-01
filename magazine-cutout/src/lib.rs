use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hash: HashMap<&str, u32> = HashMap::new();
    for &word in magazine {
        *hash.entry(&word).or_insert(0) += 1;
    }
    for &word in note {
        match hash.get_mut(word) {
            Some(wc) =>
                if *wc > 0 {
                    *wc -= 1
                } else {
                    return false;
                }
            None => return false
        }
    }
    true
}
