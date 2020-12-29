use crate::day::Day;

pub struct Day07 {}

fn split_by_brackets(s: &str) -> (Vec<&str>, Vec<&str>) {
    let open_pos: Vec<_> = s
        .chars()
        .enumerate()
        .filter(|(_, x)| *x == '[')
        .map(|(x, _)| x)
        .collect();
    let close_pos: Vec<_> = s
        .chars()
        .enumerate()
        .filter(|(_, x)| *x == ']')
        .map(|(x, _)| x)
        .collect();
    assert_eq!(open_pos.len(), close_pos.len());

    let inside_slices: Vec<_> = open_pos
        .iter()
        .zip(close_pos.iter())
        .map(|(o, c)| &s[o + 1..*c])
        .collect();

    let mut outside_slices = vec![&s[..open_pos[0]]];
    for i in 1..open_pos.len() {
        outside_slices.push(&s[close_pos[i - 1] + 1..open_pos[i]]);
    }
    outside_slices.push(&s[close_pos[close_pos.len() - 1] + 1..]);
    (inside_slices, outside_slices)
}

fn has_abba(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    chars
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn find_aba(s: &str) -> Vec<(char, char)> {
    let chars: Vec<_> = s.chars().collect();
    chars
        .windows(3)
        .map(|w| {
            if w[0] == w[2] && w[0] != w[1] {
                Some((w[0], w[1]))
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect()
}

fn has_bab(s: &str, a: char, b: char) -> bool {
    let chars: Vec<_> = s.chars().collect();
    chars
        .windows(3)
        .any(|w| w[0] == b && w[1] == a && w[2] == b)
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let num_tls_ips = input
            .lines()
            .filter(|line| {
                let (inside_slices, outside_slices) = split_by_brackets(line);
                outside_slices.iter().any(|s| has_abba(s))
                    && !inside_slices.iter().any(|s| has_abba(s))
            })
            .count();
        format!("{}", num_tls_ips)
    }

    fn star2(&self, input: &str) -> String {
        let num_ssl_ips = input
            .lines()
            .filter(|line| {
                let (inside_slices, outside_slices) = split_by_brackets(line);
                let abas: Vec<(char, char)> = outside_slices
                    .iter()
                    .map(|sl| find_aba(sl))
                    .flatten()
                    .collect();
                abas.iter()
                    .any(|x| inside_slices.iter().any(|s| has_bab(s, x.0, x.1)))
            })
            .count();
        format!("{}", num_ssl_ips)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day07 {};
        assert_eq!(d.star1(r#"abba[mnop]qrst"#), "1");
        assert_eq!(d.star1(r#"abcd[bddb]xyyx"#), "0");
        assert_eq!(d.star1(r#"aaaa[qwer]tyui"#), "0");
        assert_eq!(d.star1(r#"ioxxoj[asdfgh]zxcvbn"#), "1");
    }

    #[test]
    fn star2() {
        let d = Day07 {};
        assert_eq!(d.star2(r#"aba[bab]xyz"#), "1");
        assert_eq!(d.star2(r#"xyx[xyx]xyx"#), "0");
        assert_eq!(d.star2(r#"aaa[kek]eke"#), "1");
        assert_eq!(d.star2(r#"zazbz[bzb]cdb"#), "1");
    }
}
