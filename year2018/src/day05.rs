use common::day::Day;

pub struct Day05 {}

fn reduce(s: &str) -> String {
    let mut output = vec![];
    for c in s.chars() {
        if output.is_empty() {
            output.push(c);
        } else {
            let last = output.last().unwrap();
            if last.eq_ignore_ascii_case(&c) && *last != c {
                output.pop();
            } else {
                output.push(c);
            }
        }
    }
    output.into_iter().collect()
}

fn remove_pair(s: &str, rm_char: char) -> String {
    s.replace(|c: char| c.to_ascii_lowercase() == rm_char, "")
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let reduced = reduce(input.trim());
        format!("{}", reduced.len())
    }

    fn star2(&self, input: &str) -> String {
        let shortest = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|rm_char| {
                let removed = remove_pair(input.trim(), rm_char);
                let reduced = reduce(&removed);
                reduced.len()
            })
            .min()
            .unwrap();
        format!("{}", shortest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day05 {};
        assert_eq!(d.star1("dabAcCaCBAcCcaDA"), "10");
        assert_eq!(d.star2("dabAcCaCBAcCcaDA"), "4");
    }
}
