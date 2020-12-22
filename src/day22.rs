use crate::day::Day;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day22 {}

pub enum Winner {
    P1,
    P2,
}

fn parse_input(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player_stacks: Vec<_> = input
        .split("\n\n")
        .map(|sec| {
            let stack: VecDeque<usize> = sec
                .lines()
                .map(|l| l.parse::<usize>())
                .filter_map(Result::ok)
                .collect();
            stack
        })
        .collect();
    let p2 = player_stacks.pop().unwrap();
    let p1 = player_stacks.pop().unwrap();
    (p1, p2)
}

fn play(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> (Winner, VecDeque<usize>, VecDeque<usize>) {
    while !p1.is_empty() && !p2.is_empty() {
        let played_p1 = p1.pop_front().unwrap();
        let played_p2 = p2.pop_front().unwrap();
        if played_p1 > played_p2 {
            p1.push_back(played_p1);
            p1.push_back(played_p2);
        } else {
            p2.push_back(played_p2);
            p2.push_back(played_p1);
        }
    }
    if !p1.is_empty() {
        (Winner::P1, p1, p2)
    } else {
        (Winner::P2, p1, p2)
    }
}

fn played_state(p1: &VecDeque<usize>, p2: &VecDeque<usize>) -> String {
    let p1s = p1.iter().map(|x| x.to_string()).join(",");
    let p2s = p2.iter().map(|x| x.to_string()).join(",");
    format!("{}:{}", p1s, p2s)
}

fn play_recurse(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> (Winner, VecDeque<usize>, VecDeque<usize>) {
    let mut played_games = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let state = played_state(&p1, &p2);
        if played_games.contains(&state) {
            return (Winner::P1, p1, p2);
        }
        played_games.insert(state);

        let played_p1 = p1.pop_front().unwrap();
        let played_p2 = p2.pop_front().unwrap();

        if played_p1 > p1.len() || played_p2 > p2.len() {
            // play regular
            if played_p1 > played_p2 {
                p1.push_back(played_p1);
                p1.push_back(played_p2);
            } else {
                p2.push_back(played_p2);
                p2.push_back(played_p1);
            }
        } else {
            // play recursive
            let rec_p1: VecDeque<usize> = (0..played_p1).map(|i| p1[i]).collect();
            let rec_p2: VecDeque<usize> = (0..played_p2).map(|i| p2[i]).collect();
            let (winner, _, _) = play_recurse(rec_p1, rec_p2);
            match winner {
                Winner::P1 => {
                    p1.push_back(played_p1);
                    p1.push_back(played_p2);
                }
                Winner::P2 => {
                    p2.push_back(played_p2);
                    p2.push_back(played_p1);
                }
            }
        }
    }
    if !p1.is_empty() {
        (Winner::P1, p1, p2)
    } else {
        (Winner::P2, p1, p2)
    }
}

fn score(stack: &VecDeque<usize>) -> usize {
    stack
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, card)| (idx + 1) * card)
        .sum()
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let (p1, p2) = parse_input(input);
        let (winner, p1, p2) = play(p1, p2);
        match winner {
            Winner::P1 => format!("{}", score(&p1)),
            Winner::P2 => format!("{}", score(&p2)),
        }
    }

    fn star2(&self, input: &str) -> String {
        let (p1, p2) = parse_input(input);
        let (winner, p1, p2) = play_recurse(p1, p2);
        match winner {
            Winner::P1 => format!("{}", score(&p1)),
            Winner::P2 => format!("{}", score(&p2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let d = Day22 {};
        assert_eq!(d.star1(input), "306");
        assert_eq!(d.star2(input), "291");
    }
}
