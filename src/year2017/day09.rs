use crate::day::Day;

pub struct Day09 {}

fn score_count(s: &str) -> (usize, usize) {
    let mut stack = vec![];
    let mut it = s.chars();
    let mut total_score = 0;
    let mut garbage_count = 0;

    while let Some(c) = it.next() {
        match c {
            '{' => {
                if stack.is_empty() || stack.last().unwrap() != &'<' {
                    stack.push('{');
                } else if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    garbage_count += 1;
                }
            }
            '}' => {
                if !stack.is_empty() && stack.last().unwrap() == &'{' {
                    let group_score = stack.iter().filter(|c| c == &&'{').count();
                    total_score += group_score;
                    stack.pop();
                } else if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    garbage_count += 1;
                }
            }
            '<' => {
                if stack.is_empty() || stack.last().unwrap() != &'<' {
                    stack.push('<');
                } else if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    garbage_count += 1;
                }
            }
            '>' => {
                if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    stack.pop();
                }
            }
            '!' => {
                if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    it.next();
                }
            }
            _ => {
                if !stack.is_empty() && stack.last().unwrap() == &'<' {
                    garbage_count += 1;
                }
            }
        }
    }
    (total_score, garbage_count)
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        format!("{}", score_count(input.trim()).0)
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", score_count(input.trim()).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        assert_eq!(score_count("{}").0, 1);
        assert_eq!(score_count("{{{}}}").0, 6);
        assert_eq!(score_count("{{},{}}").0, 5);
        assert_eq!(score_count("{{{},{},{{}}}}").0, 16);
        assert_eq!(score_count("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(score_count("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(score_count("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score_count("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn star2() {
        assert_eq!(score_count("<>").1, 0);
        assert_eq!(score_count("<random characters>").1, 17);
        assert_eq!(score_count("<<<<>").1, 3);
        assert_eq!(score_count("<{!>}>").1, 2);
        assert_eq!(score_count("<!!>").1, 0);
        assert_eq!(score_count("<!!!>>").1, 0);
        assert_eq!(score_count(r#"<{o"i!a,<{i<a>"#).1, 10);
    }
}
