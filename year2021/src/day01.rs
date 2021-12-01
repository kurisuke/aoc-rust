use common::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let depths = parse_input(input);
        let num_increases = depths.windows(2).filter(|d| d[1] > d[0]).count();
        format!("{}", num_increases)
    }

    fn star2(&self, input: &str) -> String {
        let depths = parse_input(input);
        let windows3: Vec<_> = depths.windows(3).map(|d| d[0] + d[1] + d[2]).collect();
        let num_increases = windows3.windows(2).filter(|d| d[1] > d[0]).count();
        format!("{}", num_increases)
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse::<u64>())
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
199
200
208
210
200
207
240
269
260
263
"#;

        let d = Day01 {};
        assert_eq!(d.star1(input), "7");
        assert_eq!(d.star2(input), "5");
    }
}
