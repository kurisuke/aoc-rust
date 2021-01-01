use crate::day::Day;
use regex::Regex;

pub struct Day21 {}

#[derive(Debug)]
enum Op {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotPos(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn revert(s: &mut Vec<char>, op: Op) {
    match op {
        Op::SwapPos(x, y) => {
            s.swap(x, y);
        }
        Op::SwapLetter(x, y) => {
            let pos_x = s.iter().position(|&c| c == x).unwrap();
            let pos_y = s.iter().position(|&c| c == y).unwrap();
            s.swap(pos_x, pos_y);
        }
        Op::RotLeft(x) => {
            s.rotate_right(x);
        }
        Op::RotRight(x) => {
            s.rotate_left(x);
        }
        Op::RotPos(x) => {
            let pos_x = s.iter().position(|&c| c == x).unwrap();
            let r = match pos_x {
                1 => 1,
                3 => 2,
                5 => 3,
                7 => 4,
                2 => 6,
                4 => 7,
                6 => 0,
                0 => 1,
                _ => {
                    panic!("Unexpected position: {}", pos_x);
                }
            };
            s.rotate_left(r);
        }
        Op::Reverse(x, y) => {
            s[x..=y].reverse();
        }
        Op::Move(x, y) => {
            let m = s.remove(y);
            s.insert(x, m);
        }
    }
}

fn apply(s: &mut Vec<char>, op: Op) {
    match op {
        Op::SwapPos(x, y) => {
            s.swap(x, y);
        }
        Op::SwapLetter(x, y) => {
            let pos_x = s.iter().position(|&c| c == x).unwrap();
            let pos_y = s.iter().position(|&c| c == y).unwrap();
            s.swap(pos_x, pos_y);
        }
        Op::RotLeft(x) => {
            s.rotate_left(x);
        }
        Op::RotRight(x) => {
            s.rotate_right(x);
        }
        Op::RotPos(x) => {
            let pos_x = s.iter().position(|&c| c == x).unwrap();
            let r = if pos_x < 4 { 1 + pos_x } else { 2 + pos_x };
            let r = r % s.len();
            s.rotate_right(r);
        }
        Op::Reverse(x, y) => {
            s[x..=y].reverse();
        }
        Op::Move(x, y) => {
            let m = s.remove(x);
            s.insert(y, m);
        }
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    let re_swap_pos = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re_swap_letter = Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
    let re_rotate = Regex::new(r"rotate (left|right) (\d+) step").unwrap();
    let re_rotate_pos = Regex::new(r"rotate based on position of letter (\w+)").unwrap();
    let re_reverse = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_move = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            if let Some(c) = re_swap_pos.captures(line) {
                let x = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                Op::SwapPos(x, y)
            } else if let Some(c) = re_swap_letter.captures(line) {
                let x = c.get(1).unwrap().as_str().chars().next().unwrap();
                let y = c.get(2).unwrap().as_str().chars().next().unwrap();
                Op::SwapLetter(x, y)
            } else if let Some(c) = re_rotate.captures(line) {
                let lr = c.get(1).unwrap().as_str();
                let x = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                match lr {
                    "left" => Op::RotLeft(x),
                    "right" => Op::RotRight(x),
                    _ => {
                        panic!("Cannot parse line: {}", line);
                    }
                }
            } else if let Some(c) = re_rotate_pos.captures(line) {
                let x = c.get(1).unwrap().as_str().chars().next().unwrap();
                Op::RotPos(x)
            } else if let Some(c) = re_reverse.captures(line) {
                let x = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                Op::Reverse(x, y)
            } else if let Some(c) = re_move.captures(line) {
                let x = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let y = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                Op::Move(x, y)
            } else {
                panic!("Cannot parse line: {}", line);
            }
        })
        .collect()
}

fn apply_cmds(input: &str, ops: Vec<Op>) -> String {
    let mut s: Vec<_> = input.chars().collect();
    for op in ops {
        apply(&mut s, op);
    }
    s.into_iter().collect()
}

fn revert_cmds(input: &str, ops: Vec<Op>) -> String {
    let mut s: Vec<_> = input.chars().collect();
    for op in ops.into_iter().rev() {
        revert(&mut s, op);
    }
    s.into_iter().collect()
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let ops = parse_input(input);
        apply_cmds("abcdefgh", ops)
    }

    fn star2(&self, input: &str) -> String {
        let ops = parse_input(input);
        revert_cmds("fbgdceah", ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#;
        let ops = parse_input(input);
        assert_eq!(apply_cmds("abcde", ops), "decab");
    }
}
