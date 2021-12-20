use common::day::Day;
use util::intcode::Intcode;

pub struct Day07 {}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn sum_distances(pos: &[i64], target: i64) -> i64 {
    pos.iter().map(|x| (x - target).abs()).sum()
}

fn sum_distances_part2(pos: &[i64], target: i64) -> i64 {
    pos.iter().map(|x| sum_seq((x - target).abs())).sum()
}

fn sum_seq(n: i64) -> i64 {
    n * (n + 1) / 2
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        intcode.run();
        print!("Easter egg: {}", intcode.read_outp_ascii());

        let pos = parse_input(input);
        let pos_min = *pos.iter().min().unwrap();
        let pos_max = *pos.iter().max().unwrap();
        let min_fuel = (pos_min..=pos_max)
            .map(|t| sum_distances(&pos, t))
            .min()
            .unwrap();
        format!("{}", min_fuel)
    }

    fn star2(&self, input: &str) -> String {
        let pos = parse_input(input);
        let pos_min = *pos.iter().min().unwrap();
        let pos_max = *pos.iter().max().unwrap();
        let min_fuel = (pos_min..=pos_max)
            .map(|t| sum_distances_part2(&pos, t))
            .min()
            .unwrap();
        format!("{}", min_fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day07 {};
        assert_eq!(d.star1("16,1,2,0,4,2,7,1,2,14"), "37");
        assert_eq!(d.star2("16,1,2,0,4,2,7,1,2,14"), "168");
    }
}
