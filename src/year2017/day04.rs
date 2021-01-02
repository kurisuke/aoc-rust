use crate::day::Day;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day04 {}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let valid_phrases = input
            .lines()
            .filter(|line| {
                let words_vec: Vec<_> = line.split_whitespace().collect();
                let words_set: HashSet<_> = words_vec.iter().collect();
                words_vec.len() == words_set.len()
            })
            .count();
        format!("{}", valid_phrases)
    }

    fn star2(&self, input: &str) -> String {
        let valid_phrases = input
            .lines()
            .filter(|line| {
                let words_vec: Vec<_> = line.split_whitespace().collect();
                let words_set: HashSet<_> = words_vec
                    .iter()
                    .map(|w| w.chars().sorted().collect::<String>())
                    .collect();
                words_vec.len() == words_set.len()
            })
            .count();
        format!("{}", valid_phrases)
    }
}
