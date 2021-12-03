use common::day::Day;

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let (numbers, width) = parse_input(input);
        let (gamma, epsilon) = gamma_epsilon(&numbers, width);

        format!("{}", gamma * epsilon)
    }

    fn star2(&self, input: &str) -> String {
        let (numbers, width) = parse_input(input);

        let o2 = filter_numbers(&numbers, width, true).unwrap();
        let co2 = filter_numbers(&numbers, width, false).unwrap();

        format!("{}", o2 * co2)
    }
}

fn most_common_at_pos(numbers: &[u64], w: usize) -> u64 {
    let sum_ones = numbers.iter().filter(|n| *n & (1 << w) > 0).count();
    if sum_ones * 2 >= numbers.len() {
        1
    } else {
        0
    }
}

fn gamma_epsilon(numbers: &[u64], width: usize) -> (u64, u64) {
    let mut gamma = 0;
    let mut epsilon = 0;

    for w in 0..width {
        if most_common_at_pos(numbers, w) == 1 {
            gamma += 1 << w;
        } else {
            epsilon += 1 << w;
        }
    }

    (gamma, epsilon)
}

fn filter_numbers(numbers: &[u64], width: usize, most_common: bool) -> Option<u64> {
    let mut numbers = numbers.to_vec();
    for w in (0..width).rev() {
        let c = most_common_at_pos(&numbers, w);
        if most_common {
            numbers = numbers
                .into_iter()
                .filter(|n| {
                    let n_bit = (*n & (1 << w)) >> w;
                    n_bit == c
                })
                .collect();
        } else {
            numbers = numbers
                .into_iter()
                .filter(|n| {
                    let n_bit = (*n & (1 << w)) >> w;
                    n_bit != c
                })
                .collect();
        }
        if numbers.len() == 1 {
            return Some(numbers[0]);
        }
    }
    None
}

fn parse_input(input: &str) -> (Vec<u64>, usize) {
    let width = input.lines().find(|x| !x.is_empty()).unwrap().len();

    (
        input
            .lines()
            .map(|l| u64::from_str_radix(l, 2))
            .filter_map(Result::ok)
            .collect(),
        width,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

        let d = Day03 {};
        assert_eq!(d.star1(input), "198");
        assert_eq!(d.star2(input), "230");
    }
}
