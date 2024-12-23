use common::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        parse_input(input)
            .map(|init| SecretNumber::new(init).nth(2000).unwrap())
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let prices_per_buyer: Vec<_> = parse_input(input)
            .map(|init| SecretNumber::new(init).map(|i| i % 10).take(2001).collect())
            .collect();
        let mut bananas_per_sequence = vec![0; ARRAY_SIZE];
        let mut seen = vec![0; ARRAY_SIZE];
        for (i, prices) in prices_per_buyer.into_iter().enumerate() {
            diff_sequences(i + 1, prices, &mut bananas_per_sequence, &mut seen);
        }

        let most_bananas = bananas_per_sequence.iter().max().unwrap();

        most_bananas.to_string()
    }
}

const ARRAY_SIZE: usize = 1 << 20;

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

struct SecretNumber {
    num: usize,
}

impl SecretNumber {
    fn new(init: usize) -> Self {
        Self { num: init }
    }
}

impl Iterator for SecretNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.num;
        self.num = ((self.num << 6) ^ self.num) % 16777216;
        self.num = ((self.num >> 5) ^ self.num) % 16777216;
        self.num = ((self.num << 11) ^ self.num) % 16777216;

        Some(ret)
    }
}

fn diff_sequences(
    i: usize,
    prices: Vec<usize>,
    bananas_per_sequence: &mut [usize],
    seen: &mut [usize],
) {
    for p in prices.windows(5) {
        let key = ((9 + p[1] - p[0]) << 15)
            + ((9 + p[2] - p[1]) << 10)
            + ((9 + p[3] - p[2]) << 5)
            + (9 + p[4] - p[3]);

        if seen[key] != i {
            seen[key] = i;
            bananas_per_sequence[key] += p[4];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"1
10
100
2024"#;

    const INPUT2: &str = r#"1
2
3
2024"#;

    #[test]
    fn star1() {
        let d = Day22 {};
        assert_eq!(d.star1(INPUT1), "37327623");
    }

    #[test]
    fn star2() {
        let d = Day22 {};
        assert_eq!(d.star2(INPUT2), "23");
    }
}
