use common::day::Day;
use std::collections::HashSet;

pub struct Day24 {}

type Components = HashSet<(usize, usize)>;

fn max_strength(unused: Components, last: usize) -> usize {
    let mut max = 0;
    for comp in unused.iter().filter(|x| x.0 == last || x.1 == last) {
        let mut new_unused = unused.clone();
        new_unused.remove(comp);
        let other = if last == comp.0 { comp.1 } else { comp.0 };
        let strength = comp.0 + comp.1 + max_strength(new_unused, other);
        max = max.max(strength);
    }
    max
}

fn max_len_strength(unused: Components, last: usize, len: usize) -> (usize, usize) {
    let mut max_len = len;
    let mut max_strength = 0;
    for comp in unused.iter().filter(|x| x.0 == last || x.1 == last) {
        let mut new_unused = unused.clone();
        new_unused.remove(comp);
        let other = if last == comp.0 { comp.1 } else { comp.0 };

        let (mut strength, new_len) = max_len_strength(new_unused, other, len + 1);
        strength += comp.0 + comp.1;
        if new_len > max_len || (new_len == max_len && strength > max_strength) {
            max_len = new_len;
            max_strength = strength;
        }
    }
    (max_strength, max_len)
}

fn parse_input(input: &str) -> Components {
    input
        .lines()
        .map(|line| {
            let spl: Vec<_> = line.split('/').collect();
            (
                spl[0].parse::<usize>().unwrap(),
                spl[1].parse::<usize>().unwrap(),
            )
        })
        .collect()
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let components = parse_input(input);
        format!("{}", max_strength(components, 0))
    }

    fn star2(&self, input: &str) -> String {
        let components = parse_input(input);
        format!("{}", max_len_strength(components, 0, 0).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day24 {};
        let input = r#"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"#;
        assert_eq!(d.star1(input), "31");
        assert_eq!(d.star2(input), "19");
    }
}
