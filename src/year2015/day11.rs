use crate::day::Day;
use std::collections::HashSet;

pub struct Day11 {}

fn increment_pw(pw: &str) -> String {
    let mut chars: Vec<_> = pw.chars().collect();
    let mut i = chars.len() - 1;
    let mut done = false;
    while !done {
        if chars[i] != 'z' {
            chars[i] = (chars[i] as u8 + 1) as char;
            done = true;
        } else if i > 0 {
            chars[i] = 'a';
            i -= 1;
        } else {
            done = true;
        }
    }
    chars.into_iter().collect()
}

fn is_valid_pw(pw: &str) -> bool {
    let chars: Vec<_> = pw.chars().collect();
    let cond1 = chars
        .windows(3)
        .any(|w| w[1] as u8 == w[0] as u8 + 1 && w[2] as u8 == w[1] as u8 + 1);
    let cond2 = chars.iter().all(|&c| c != 'l' && c != 'i' && c != 'o');
    let pairs: HashSet<_> = chars
        .windows(2)
        .filter(|w| w[0] == w[1])
        .map(|w| w[1])
        .collect();
    let cond3 = pairs.len() >= 2;
    cond1 && cond2 && cond3
}

fn next_valid_pw(pw: &str) -> String {
    let mut check_pw = increment_pw(pw);
    while !is_valid_pw(&check_pw) {
        check_pw = increment_pw(&check_pw);
    }
    check_pw
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        next_valid_pw(&input)
    }

    fn star2(&self, input: &str) -> String {
        next_valid_pw(&next_valid_pw(&input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment_pw("xyy"), "xyz");
        assert_eq!(increment_pw("xyz"), "xza");
        assert_eq!(increment_pw("xzz"), "yaa");
        assert_eq!(increment_pw("abcdefgh"), "abcdefgi");
    }

    #[test]
    fn test_valid() {
        assert_eq!(is_valid_pw("hijklmmn"), false);
        assert_eq!(is_valid_pw("abbceffg"), false);
        assert_eq!(is_valid_pw("abbcegjk"), false);
        assert_eq!(is_valid_pw("abcdffaa"), true);
        assert_eq!(is_valid_pw("ghjaabcc"), true);
    }
}
