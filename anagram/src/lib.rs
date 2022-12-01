use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let word_lowercase = word.to_lowercase();
    let mut word_sorted = word_lowercase.chars().collect::<Vec<char>>();
    word_sorted.sort_unstable();
    for &candidate in possible_anagrams {
        let candidate_lowercase = candidate.to_lowercase();
        let mut candidate_sorted = candidate_lowercase.chars().collect::<Vec<char>>();
        candidate_sorted.sort_unstable();
        if candidate_sorted == word_sorted && word_lowercase != candidate_lowercase {
            anagrams.insert(candidate);
        }
    }
    anagrams
}
