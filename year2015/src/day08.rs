use common::day::Day;

pub struct Day08 {}

fn mem_diff(s: &str) -> usize {
    let chars: Vec<_> = s.chars().collect();
    let mut diff = 0;

    let mut it = chars.iter().skip(1);
    while let Some(c1) = it.next() {
        if *c1 == '\\' {
            let c2 = it.next().unwrap_or_else(|| panic!("Parsing error: {}", s));
            match c2 {
                '\\' => diff += 1,
                '"' => diff += 1,
                'x' => {
                    let c3 = it.next().unwrap_or_else(|| panic!("Parsing error: {}", s));
                    let c4 = it.next().unwrap_or_else(|| panic!("Parsing error: {}", s));
                    if c3.is_digit(16) && c4.is_digit(16) {
                        diff += 3;
                    } else {
                        panic!("Parsing error: {}", s)
                    }
                }
                _ => {
                    panic!("Parsing error: {}", s)
                }
            }
        }
    }
    diff + 2
}

fn encode_diff(s: &str) -> usize {
    let backslashes = s.chars().filter(|c| *c == '\\').count();
    let quotes = s.chars().filter(|c| *c == '"').count();
    backslashes + quotes + 2
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let total: usize = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(mem_diff)
            .sum();
        format!("{}", total)
    }

    fn star2(&self, input: &str) -> String {
        let total: usize = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(encode_diff)
            .sum();
        format!("{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(mem_diff(r#""""#), 2);
        assert_eq!(mem_diff(r#"abc"#), 2);
        assert_eq!(mem_diff(r#""aaa\"aaa""#), 3);
        assert_eq!(mem_diff(r#""\x27""#), 5);
    }

    #[test]
    fn part1() {
        let d = Day08 {};
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(d.star1(input), "12");
    }

    #[test]
    fn part2() {
        let d = Day08 {};
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(d.star2(input), "19");
    }
}
