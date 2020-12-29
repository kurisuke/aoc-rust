use crate::day::Day;
use num_complex::Complex;
use std::collections::HashSet;

pub struct Day01 {}

enum LR {
    L,
    R,
}

fn parse_input(input: &str) -> Vec<(LR, isize)> {
    input
        .split(", ")
        .map(|cmd| {
            if cmd.len() < 2 {
                None
            } else {
                let lr = match cmd.chars().next().unwrap() {
                    'L' => LR::L,
                    'R' => LR::R,
                    _ => {
                        panic!("Invalid command: {}", cmd);
                    }
                };
                let n = cmd[1..].trim().parse::<isize>().unwrap();
                Some((lr, n))
            }
        })
        .filter_map(|x| x)
        .collect()
}

fn move_pos(cmds: &[(LR, isize)]) -> isize {
    let mut pos = Complex::<isize>::new(0, 0);
    let mut vec = Complex::<isize>::new(0, 1);
    for cmd in cmds {
        match cmd.0 {
            LR::L => {
                vec *= Complex::<isize>::new(0, 1);
            }
            LR::R => {
                vec *= Complex::<isize>::new(0, -1);
            }
        }
        pos += cmd.1 * vec;
    }
    pos.re.abs() + pos.im.abs()
}

fn move_pos_star2(cmds: &[(LR, isize)]) -> isize {
    let mut pos = Complex::<isize>::new(0, 0);
    let mut vec = Complex::<isize>::new(0, 1);
    let mut pos_hist = HashSet::new();
    pos_hist.insert(pos);
    for cmd in cmds {
        match cmd.0 {
            LR::L => {
                vec *= Complex::<isize>::new(0, 1);
            }
            LR::R => {
                vec *= Complex::<isize>::new(0, -1);
            }
        }
        for _ in 0..cmd.1 {
            pos += vec;
            if !pos_hist.insert(pos) {
                return pos.re.abs() + pos.im.abs();
            }
        }
    }
    -1
}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        format!("{}", move_pos(&cmds))
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        format!("{}", move_pos_star2(&cmds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day01 {};
        assert_eq!(d.star1("R2, L3"), "5");
        assert_eq!(d.star1("R2, R2, R2"), "2");
        assert_eq!(d.star1("R5, L5, R5, R3"), "12");
    }

    #[test]
    fn star2() {
        let d = Day01 {};
        assert_eq!(d.star2("R8, R4, R4, R8"), "4");
    }
}
