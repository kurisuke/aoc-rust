use common::day::Day;
use regex::Regex;

pub struct Day15 {}

struct Disc {
    div: usize,
    off: usize,
}

fn check_fall(discs: &[Disc], t: usize) -> bool {
    discs
        .iter()
        .enumerate()
        .all(|(i, disc)| (disc.off + t + i + 1) % disc.div == 0)
}

fn parse_input(input: &str) -> Vec<Disc> {
    let re_div = Regex::new(r"has (\d+) positions").unwrap();
    let re_off = Regex::new(r"it is at position (\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps_div = re_div.captures(line).unwrap();
            let div = caps_div.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let caps_off = re_off.captures(line).unwrap();
            let off = caps_off.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Disc { div, off }
        })
        .collect()
}

fn press_time(discs: &[Disc]) -> usize {
    let mut t = 0;
    while !check_fall(&discs, t) {
        t += 1;
    }
    t
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let discs = parse_input(input);
        format!("{}", press_time(&discs))
    }

    fn star2(&self, input: &str) -> String {
        let mut discs = parse_input(input);
        discs.push(Disc { div: 11, off: 0 });
        format!("{}", press_time(&discs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day15 {};
        let input = r#"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."#;
        assert_eq!(d.star1(input), "5");
    }
}
