use crate::day::Day;
use std::collections::HashMap;

pub struct Day15 {}

fn find_num(start_nums: &[u64], n: usize) -> u64 {
    let mut prev_turn = HashMap::new();

    for (i, n) in start_nums.iter().enumerate().take(start_nums.len() - 1) {
        prev_turn.insert(*n, i as usize);
    }
    let mut last_spoken = *start_nums.last().unwrap();
    let mut turn = start_nums.len() - 1;

    while turn < n - 1 {
        let next_spoken = match prev_turn.get(&last_spoken) {
            None => 0u64,
            Some(t) => (turn - t) as u64,
        };
        let e = prev_turn.entry(last_spoken).or_insert(0);
        *e = turn;

        // next turn
        last_spoken = next_spoken;
        turn += 1;
    }

    last_spoken
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        format!("{}", find_num(&start_nums, 2020))
    }

    fn star2(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        format!("{}", find_num(&start_nums, 30000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day15 {};
        assert_eq!(d.star1("0,3,6"), "436");
        assert_eq!(d.star1("1,3,2"), "1");
        assert_eq!(d.star1("2,1,3"), "10");
        assert_eq!(d.star1("1,2,3"), "27");
        assert_eq!(d.star1("2,3,1"), "78");
        assert_eq!(d.star1("3,2,1"), "438");
        assert_eq!(d.star1("3,1,2"), "1836");
    }
}
