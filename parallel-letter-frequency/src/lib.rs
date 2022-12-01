use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = Vec::with_capacity(worker_count);
    for lines in input.chunks(input.len() / worker_count + 1) {
        let string = lines.join("");
        let t = thread::spawn(move || {
            let mut hash: HashMap<char, usize> = HashMap::new();
            let chars = string
                .chars()
                .filter(|s| s.is_alphabetic())
                .map(|c| c.to_ascii_lowercase());
            for c in chars {
                if c.is_alphabetic() {
                    *hash.entry(c).or_default() += 1;
                }
            }
            hash
        });
        handles.push(t);
    }

    let mut hm = HashMap::new();
    for h in handles {
        h.join().unwrap().into_iter().for_each(|(k,v)| {
            *hm.entry(k).or_default() += v;
        })
    }
    hm
}
