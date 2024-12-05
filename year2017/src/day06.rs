use common::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day06 {}

fn cycle(banks: &mut [usize]) {
    let len = banks.len();
    let max_val = *banks.iter().max().unwrap();
    let max_pos = banks.iter().position(|v| *v == max_val).unwrap();
    banks[max_pos] = 0;
    for i in 1..=max_val {
        banks[(max_pos + i) % len] += 1;
    }
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let mut banks: Vec<_> = input
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let mut configs = HashSet::new();
        configs.insert(banks.clone());
        loop {
            cycle(&mut banks);
            if !configs.insert(banks.clone()) {
                break;
            }
        }
        format!("{}", configs.len())
    }

    fn star2(&self, input: &str) -> String {
        let mut banks: Vec<_> = input
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let mut configs = HashMap::new();
        let mut i = 0;
        configs.insert(banks.clone(), i);
        loop {
            i += 1;
            cycle(&mut banks);
            if configs.contains_key(&banks) {
                return format!("{}", i - configs.get(&banks).unwrap());
            } else {
                configs.insert(banks.clone(), i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day06 {};
        assert_eq!(d.star1("0 2 7 0"), "5");
        assert_eq!(d.star2("0 2 7 0"), "4");
    }
}
