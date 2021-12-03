use common::day::Day;

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let (numbers, width) = parse_input(input);
        let mut sums = vec![0; width];

        for n in numbers.iter() {
            for (w, sum) in sums.iter_mut().enumerate() {
                if n & (1 << w) > 0 {
                    *sum += 1;
                }
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        for (w, sum) in sums.iter().enumerate() {
            if *sum > numbers.len() / 2 {
                gamma += 1 << w;
            } else {
                epsilon += 1 << w;
            }
        }

        format!("{}", gamma * epsilon)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
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
    }
}
