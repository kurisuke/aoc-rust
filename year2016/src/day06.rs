use common::day::Day;
use std::collections::HashMap;

pub struct Day06 {}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn msg_col(msgs: &[Vec<char>], n: usize) -> Vec<char> {
    msgs.iter().map(|msg| msg[n]).collect()
}

fn letter_frequency(chars: &[char]) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in chars {
        let e = freq.entry(*c).or_insert(0);
        *e += 1;
    }
    freq
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let msgs = parse_input(input);
        let mut s = String::new();
        for col in 0..msgs[0].len() {
            let freq = letter_frequency(&msg_col(&msgs, col));
            let most_common = freq.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0;
            s.push(*most_common);
        }
        s
    }

    fn star2(&self, input: &str) -> String {
        let msgs = parse_input(input);
        let mut s = String::new();
        for col in 0..msgs[0].len() {
            let freq = letter_frequency(&msg_col(&msgs, col));
            let most_common = freq.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0;
            s.push(*most_common);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day06 {};
        let input = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;
        assert_eq!(d.star1(input), "easter");
        assert_eq!(d.star2(input), "advent");
    }
}
