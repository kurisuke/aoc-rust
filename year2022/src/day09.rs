use common::day::Day;
use std::collections::HashSet;

pub struct Day09 {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(i64, i64);

impl Pos {
    fn moves(&self, other: &Pos) -> u64 {
        (self.0 - other.0)
            .unsigned_abs()
            .max((self.1 - other.1).unsigned_abs())
    }
}

struct MoveCmd {
    dir: char,
    repeat: usize,
}

fn parse_input(input: &str) -> impl Iterator<Item = MoveCmd> + '_ {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        let dir = tokens.next().unwrap().chars().next().unwrap();
        let repeat = tokens.next().unwrap().parse().unwrap();
        MoveCmd { dir, repeat }
    })
}

fn run(input: &str, len: usize) -> usize {
    let cmds = parse_input(input);
    let mut knots = vec![Pos(0, 0); len];
    let mut tail_pos = HashSet::new();

    for cmd in cmds {
        for _ in 0..cmd.repeat {
            match cmd.dir {
                'L' => {
                    knots[0].0 -= 1;
                }
                'R' => {
                    knots[0].0 += 1;
                }
                'U' => {
                    knots[0].1 -= 1;
                }
                'D' => {
                    knots[0].1 += 1;
                }
                _ => unreachable!(),
            }
            for i in 1..len {
                let delta_moves = knots[i - 1].moves(&knots[i]);
                if delta_moves > 1 {
                    knots[i].0 += (knots[i - 1].0 - knots[i].0).signum();
                    knots[i].1 += (knots[i - 1].1 - knots[i].1).signum();
                }
            }
            tail_pos.insert(knots[len - 1]);
        }
    }
    tail_pos.len()
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        format!("{}", run(input, 2))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", run(input, 10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let d = Day09 {};
        assert_eq!(d.star1(input), "13");
        assert_eq!(d.star2(input), "1");
    }

    #[test]
    fn ex2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let d = Day09 {};
        assert_eq!(d.star2(input), "36");
    }
}
