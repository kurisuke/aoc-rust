use std::collections::HashMap;

use common::day::Day;

pub struct Day11 {}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let mut stones = parse_input(input);
        for _ in 0..25 {
            stones = blink(stones);
        }
        format!("{}", stones.values().sum::<usize>())
    }

    fn star2(&self, input: &str) -> String {
        let mut stones = parse_input(input);
        for _ in 0..75 {
            stones = blink(stones);
        }
        format!("{}", stones.values().sum::<usize>())
    }
}

type StoneMap = HashMap<usize, usize>;

fn parse_input(input: &str) -> StoneMap {
    let mut stones = HashMap::new();
    for s in input.split_whitespace() {
        let x = s.parse().unwrap();
        let e = stones.entry(x).or_insert(0);
        *e += 1;
    }
    stones
}

enum Stones {
    One(usize),
    Two(usize, usize),
}

fn change_stone(x: usize) -> Stones {
    if x == 0 {
        return Stones::One(1);
    }

    let s = x.to_string();
    if s.len() % 2 == 0 {
        let pivot = s.len() / 2;
        return Stones::Two(s[..pivot].parse().unwrap(), s[pivot..].parse().unwrap());
    }

    Stones::One(x * 2024)
}

fn blink(stones: StoneMap) -> StoneMap {
    let mut stones_new = HashMap::new();
    for (x, n) in stones {
        match change_stone(x) {
            Stones::One(x) => {
                let e = stones_new.entry(x).or_insert(0);
                *e += n;
            }
            Stones::Two(x1, x2) => {
                let e = stones_new.entry(x1).or_insert(0);
                *e += n;
                let e = stones_new.entry(x2).or_insert(0);
                *e += n;
            }
        }
    }
    stones_new
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn ex1() {
        let d = Day11 {};
        assert_eq!(d.star1(INPUT), "55312");
    }
}
