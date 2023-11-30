use common::day::Day;
use std::collections::HashSet;

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let mut houses = HashSet::new();
        let mut pos = (0, 0);
        houses.insert(pos);
        for c in input.chars() {
            match c {
                '^' => {
                    pos.0 += 1;
                }
                'v' => {
                    pos.0 -= 1;
                }
                '>' => {
                    pos.1 += 1;
                }
                '<' => {
                    pos.1 -= 1;
                }
                _ => {}
            }
            houses.insert(pos);
        }
        format!("{}", houses.len())
    }

    fn star2(&self, input: &str) -> String {
        let mut houses = vec![HashSet::new(), HashSet::new()];
        let mut pos = [(0, 0), (0, 0)];
        houses[0].insert(pos[0]);
        for (i, c) in input.chars().enumerate() {
            match c {
                '^' => {
                    pos[i % 2].0 += 1;
                }
                'v' => {
                    pos[i % 2].0 -= 1;
                }
                '>' => {
                    pos[i % 2].1 += 1;
                }
                '<' => {
                    pos[i % 2].1 -= 1;
                }
                _ => {}
            }
            houses[i % 2].insert(pos[i % 2]);
        }
        format!(
            "{}",
            houses[0].union(&houses[1]).collect::<HashSet<_>>().len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let d = Day03 {};
        assert_eq!(d.star1(">"), "2");
        assert_eq!(d.star1("^>v<"), "4");
        assert_eq!(d.star1("^v^v^v^v^v"), "2");
    }

    #[test]
    fn part2() {
        let d = Day03 {};
        assert_eq!(d.star2("^v"), "3");
        assert_eq!(d.star2("^>v<"), "3");
        assert_eq!(d.star2("^v^v^v^v^v"), "11");
    }
}
