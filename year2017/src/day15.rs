use common::day::Day;

pub struct Day15 {}

#[derive(Copy, Clone)]
struct Generator {
    cur: u64,
    factor: u64,
    div: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.cur = (self.cur * self.factor) % self.div;
        Some(self.cur)
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            words.last().unwrap().parse::<u64>().unwrap()
        })
        .collect()
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let start = parse_input(input);
        let gen_a = Generator {
            cur: start[0],
            factor: 16807,
            div: 2147483647,
        };
        let gen_b = Generator {
            cur: start[1],
            factor: 48271,
            div: 2147483647,
        };
        let count = gen_a
            .zip(gen_b)
            .take(40_000_000)
            .filter(|&(a, b)| a & 0xffff == b & 0xffff)
            .count();
        format!("{}", count)
    }

    fn star2(&self, input: &str) -> String {
        let start = parse_input(input);
        let gen_a = Generator {
            cur: start[0],
            factor: 16807,
            div: 2147483647,
        };
        let gen_b = Generator {
            cur: start[1],
            factor: 48271,
            div: 2147483647,
        };
        let count = gen_a
            .filter(|x| x % 4 == 0)
            .zip(gen_b.filter(|x| x % 8 == 0))
            .take(5_000_000)
            .filter(|&(a, b)| a & 0xffff == b & 0xffff)
            .count();
        format!("{}", count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day15 {};
        let input = r#"Generator A starts with 65
Generator B starts with 8921"#;
        assert_eq!(d.star1(input), "588");
        assert_eq!(d.star2(input), "309");
    }
}
