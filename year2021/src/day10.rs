use common::day::Day;

pub struct Day10 {}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_syntax_error(chars: &[char]) -> Result<Vec<char>, char> {
    let mut stack = vec![];
    for c in chars {
        match c {
            '(' | '{' | '[' | '<' => {
                stack.push(*c);
            }
            ')' => {
                let last = stack.pop();
                if last.is_some() && last.unwrap() != '(' {
                    return Err(*c);
                }
            }
            '}' => {
                let last = stack.pop();
                if last.is_some() && last.unwrap() != '{' {
                    return Err(*c);
                }
            }
            ']' => {
                let last = stack.pop();
                if last.is_some() && last.unwrap() != '[' {
                    return Err(*c);
                }
            }
            '>' => {
                let last = stack.pop();
                if last.is_some() && last.unwrap() != '<' {
                    return Err(*c);
                }
            }
            _ => {}
        }
    }
    Ok(stack)
}

fn syntax_error_score(c: &char) -> Option<u64> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn closing_characters(chars: &[char]) -> Vec<char> {
    chars
        .iter()
        .rev()
        .map(|c| match c {
            '(' => Some(')'),
            '{' => Some('}'),
            '[' => Some(']'),
            '<' => Some('>'),
            _ => None,
        })
        .flatten()
        .collect()
}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let lines = parse_input(input);
        let total_score: u64 = lines
            .iter()
            .map(|chars| find_syntax_error(chars))
            .filter_map(Result::err)
            .map(|c| syntax_error_score(&c).unwrap())
            .sum();
        format!("{}", total_score)
    }

    fn star2(&self, input: &str) -> String {
        let lines = parse_input(input);
        let mut scores: Vec<_> = lines
            .iter()
            .map(|l| match find_syntax_error(l) {
                Ok(chars) => {
                    let closing = closing_characters(&chars);
                    let mut score = 0u64;
                    for c in closing {
                        score *= 5;
                        score += match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                    }
                    Some(score)
                }
                Err(_) => None,
            })
            .flatten()
            .collect();
        scores.sort_unstable();
        println!("len: {}", scores.len());
        let middle_score = scores[scores.len() / 2];
        format!("{}", middle_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        let d = Day10 {};
        assert_eq!(d.star1(input), "26397");
        assert_eq!(d.star2(input), "288957");
    }
}
