use std::collections::HashSet;

use common::day::Day;

pub struct Day04 {}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| Card::from_line(line).points())
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut cards: Vec<_> = input
            .lines()
            .map(|line| (Card::from_line(line), 1))
            .collect();

        for i in 0..cards.len() {
            for j in 1..=cards[i].0.winning_numbers() {
                if i + j < cards.len() {
                    cards[i + j].1 += cards[i].1;
                }
            }
        }

        cards.iter().map(|(_, n)| n).sum::<usize>().to_string()
    }
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    numbers_you_have: HashSet<usize>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let line = line.split_once(':').unwrap().1;
        let (winning_str, you_have_str) = line.split_once(" | ").unwrap();
        let winning_numbers = winning_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers_you_have = you_have_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Card {
            winning_numbers,
            numbers_you_have,
        }
    }

    fn points(&self) -> usize {
        self.numbers_you_have.iter().fold(0, |acc, n| {
            if self.winning_numbers.contains(n) {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            } else {
                acc
            }
        })
    }

    fn winning_numbers(&self) -> usize {
        self.numbers_you_have
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn star1() {
        let d = Day04 {};
        assert_eq!(d.star1(INPUT), "13");
    }

    #[test]
    fn star2() {
        let d = Day04 {};
        assert_eq!(d.star2(INPUT), "30");
    }
}
