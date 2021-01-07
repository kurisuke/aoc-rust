use common::day::Day;
use util::chardistrib::char_distribution;

pub struct Day02 {}

fn occurs_n_times(s: &str, n: usize) -> bool {
    let distrib = char_distribution(s);
    distrib.values().any(|&x| x == n)
}

fn same_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter_map(|c| if c.0 == c.1 { Some(c.0) } else { None })
        .collect()
}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let twos = input.lines().filter(|line| occurs_n_times(line, 2)).count();
        let threes = input.lines().filter(|line| occurs_n_times(line, 3)).count();
        format!("{}", twos * threes)
    }

    fn star2(&self, input: &str) -> String {
        let ids: Vec<_> = input.lines().collect();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let same = same_chars(ids[i], ids[j]);
                if same.len() == ids[i].len() - 1 {
                    return same;
                }
            }
        }
        String::from("err")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day02 {};
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;
        assert_eq!(d.star1(input), "12");
    }

    #[test]
    fn star2() {
        let d = Day02 {};
        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;
        assert_eq!(d.star2(input), "fgij");
    }
}
