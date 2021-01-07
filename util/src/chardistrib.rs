use std::collections::HashMap;

pub fn char_distribution(s: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in s.chars() {
        let e = freq.entry(c).or_insert(0);
        *e += 1;
    }
    freq
}
