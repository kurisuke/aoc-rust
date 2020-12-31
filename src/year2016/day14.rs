use crate::day::Day;
use md5::{Digest, Md5};
use std::collections::BTreeMap;

pub struct Day14 {}

fn key_index(salt: &str, n: usize, stretch_factor: usize) -> usize {
    let mut i: usize = 0;
    let mut keys = vec![];
    let mut candidates: BTreeMap<usize, char> = BTreeMap::new();
    let mut hasher = Md5::new();

    while keys.len() < n || !candidates.is_empty() {
        hasher.update(salt.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize_reset();
        let mut result_str = format!("{:32x}", result);

        for _ in 0..stretch_factor {
            hasher.update(result_str.as_bytes());
            let result_stretch = hasher.finalize_reset();
            result_str = format!("{:32x}", result_stretch);
        }
        let result_chars: Vec<_> = result_str.chars().collect();

        // Prune old candidates (more than 1000 away)
        let mut to_remove = vec![];
        for k in candidates.keys() {
            if k + 1000 < i {
                to_remove.push(*k);
            } else {
                break;
            }
        }
        for k in to_remove {
            candidates.remove(&k);
        }

        // Check if current key can confirm a key candidate. Requirements:
        // - quintuple in current hash
        // - same character in candidates (triples)
        let mut found_indexes = vec![];
        if let Some(w) = result_chars
            .windows(5)
            .find(|w| w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4])
        {
            for (cand_index, repeat_char) in candidates.iter() {
                if *repeat_char == w[0] {
                    keys.push(*cand_index);
                    found_indexes.push(*cand_index);
                }
            }
        }
        for found_index in found_indexes {
            candidates.remove(&found_index);
        }

        // Check for new candidates (triple), but only if we have not yet found enough keys.
        // Otherwise we just wait for the rest of candidates to resolve, so we can get the final order.
        if keys.len() < n {
            if let Some(w) = result_chars
                .windows(3)
                .find(|w| w[0] == w[1] && w[0] == w[2])
            {
                candidates.insert(i, w[0]);
            }
        }

        i += 1;
    }

    keys.sort_unstable();
    keys[n - 1]
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        format!("{}", key_index(input, 64, 0))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", key_index(input, 64, 2016))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day14 {};
        assert_eq!(d.star1("abc"), "22728");
    }

    #[test]
    #[ignore]
    fn star2() {
        let d = Day14 {};
        assert_eq!(d.star2("abc"), "22551");
    }
}
