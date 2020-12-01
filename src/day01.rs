use crate::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let entries = parse_input(input);
        for i in 0..entries.len() {
            for j in i + 1..entries.len() {
                if entries[i] + entries[j] == 2020 {
                    return format!("{}", entries[i] * entries[j]);
                }
            }
        }
        String::from("not found")
    }

    fn star2(&self, input: &str) -> String {
        let entries = parse_input(input);
        for i in 0..entries.len() {
            for j in i + 1..entries.len() {
                for k in j + 1..entries.len() {
                    if entries[i] + entries[j] + entries[k] == 2020 {
                        return format!("{}", entries[i] * entries[j] * entries[k]);
                    }
                }
            }
        }
        String::from("not found")
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day01 {};
        let input = r#"1721
979
366
299
675
1456"#;
        assert_eq!(d.star1(input), "514579");
        assert_eq!(d.star2(input), "241861950");
    }
}
