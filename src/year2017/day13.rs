use crate::day::Day;
use std::collections::HashMap;

pub struct Day13 {}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn is_caught(t_start: usize, layers: &HashMap<usize, usize>) -> bool {
    layers.iter().any(|(t, range)| {
        let states = 2 * range - 2;
        (t_start + t) % states == 0
    })
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let layers = parse_input(input);
        let severity = layers
            .iter()
            .map(|(t, range)| {
                let states = 2 * range - 2;
                if t % states == 0 {
                    t * range
                } else {
                    0
                }
            })
            .sum::<usize>();
        format!("{}", severity)
    }

    fn star2(&self, input: &str) -> String {
        let layers = parse_input(input);
        let t_not_caught = (0..).find(|t_start| !is_caught(*t_start, &layers)).unwrap();
        format!("{}", t_not_caught)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day13 {};
        let input = r#"0: 3
1: 2
4: 4
6: 4"#;
        assert_eq!(d.star1(input), "24");
        assert_eq!(d.star2(input), "10");
    }
}
