use std::collections::HashSet;

use common::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        parse_input(input)
            .map(|init| secret_number(init, 2000))
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let prices_per_buyer: Vec<_> = parse_input(input).map(prices).collect();
        let mut seen = HashSet::new();
        let sequences_per_buyer: Vec<_> = prices_per_buyer
            .into_iter()
            .map(|prices| diff_sequences(prices, &mut seen))
            .collect();

        let most_bananas = seen
            .into_iter()
            .map(|sequence| {
                sequences_per_buyer
                    .iter()
                    .map(|buyer| buyer[sequence])
                    .sum::<usize>()
            })
            .max()
            .unwrap();

        most_bananas.to_string()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

fn secret_number(init: usize, n: usize) -> usize {
    let mut num = init;

    for _ in 0..n {
        num = ((num << 6) ^ num) % 16777216;
        num = ((num >> 5) ^ num) % 16777216;
        num = ((num << 11) ^ num) % 16777216;
    }
    num
}

fn prices(init: usize) -> Vec<i8> {
    let mut prices = vec![];

    let mut num = init;
    prices.push((num % 10) as i8);
    for _ in 0..2000 {
        num = ((num << 6) ^ num) % 16777216;
        num = ((num >> 5) ^ num) % 16777216;
        num = ((num << 11) ^ num) % 16777216;
        prices.push((num % 10) as i8);
    }
    prices
}

const ARRAY_SIZE: usize = 19 * 19 * 19 * 19;

fn diff_sequences(prices: Vec<i8>, seen: &mut HashSet<usize>) -> Vec<usize> {
    let mut sequences = vec![0; ARRAY_SIZE];
    for p in prices.windows(5) {
        let diffs = [p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]];
        let key = (diffs[0] + 9) as usize * (19 * 19 * 19)
            + (diffs[1] + 9) as usize * (19 * 19)
            + (diffs[2] + 9) as usize * 19
            + (diffs[3] + 9) as usize;
        seen.insert(key);

        sequences[key] = p[4] as usize;
    }
    sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        assert_eq!(secret_number(123, 1), 15887950);
        assert_eq!(secret_number(123, 2), 16495136);
        assert_eq!(secret_number(123, 10), 5908254);
    }

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
