use common::day::Day;

pub struct Day05 {}

fn is_nice_star1(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let num_vowels = chars
        .iter()
        .filter(|c| vowels.iter().any(|v| *c == v))
        .count();

    let has_double = chars.windows(2).any(|w| w[0] == w[1]);

    let forbidden = ["ab", "cd", "pq", "xy"];
    let has_forbidden = forbidden.iter().any(|f| s.contains(f));
    (num_vowels >= 3) && has_double && !has_forbidden
}

fn is_nice_star2(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let cond1 = chars.windows(2).enumerate().any(|(i, w)| {
        let find_str: String = w.iter().collect();
        (s[i + 2..]).contains(&find_str)
    });
    let cond2 = chars.windows(3).any(|w| w[0] == w[2]);
    cond1 && cond2
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let num_nice = input.lines().filter(|l| is_nice_star1(l)).count();
        format!("{}", num_nice)
    }

    fn star2(&self, input: &str) -> String {
        let num_nice = input.lines().filter(|l| is_nice_star2(l)).count();
        format!("{}", num_nice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        assert!(is_nice_star1("ugknbfddgicrmopn"));
        assert!(is_nice_star1("aaa"));
        assert!(!is_nice_star1("jchzalrnumimnmhp"));
        assert!(!is_nice_star1("haegwjzuvuyypxyu"));
        assert!(!is_nice_star1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn star2() {
        assert!(is_nice_star2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_star2("xxyxx"));
        assert!(!is_nice_star2("uurcxstgmygtbstg"));
        assert!(!is_nice_star2("ieodomkazucvgmuy"));
    }
}
