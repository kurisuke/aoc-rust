use common::day::Day;
use scan_fmt::{parse::ScanError, scan_fmt};

pub struct Day02 {}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let passwords = parse_input(input);
        let valid_pws = passwords
            .iter()
            .filter(|password| {
                let occur_count = password
                    .chars
                    .iter()
                    .filter(|&c| *c == password.limited_char)
                    .count();
                occur_count >= password.char_params.0 && occur_count <= password.char_params.1
            })
            .count();
        format!("{}", valid_pws)
    }

    fn star2(&self, input: &str) -> String {
        let passwords = parse_input(input);
        let valid_pws = passwords
            .iter()
            .filter(|password| {
                (password.chars[password.char_params.0 - 1] == password.limited_char)
                    ^ (password.chars[password.char_params.1 - 1] == password.limited_char)
            })
            .count();
        format!("{}", valid_pws)
    }
}

struct PwListEntry {
    pub char_params: (usize, usize),
    pub limited_char: char,
    pub chars: Vec<char>,
}

fn parse_input(input: &str) -> Vec<PwListEntry> {
    input
        .lines()
        .map::<Result<PwListEntry, ScanError>, _>(|line| {
            let (char_param1, char_param2, limited_char, pw_str) =
                scan_fmt!(line, "{d}-{d} {[a-z]}: {}", usize, usize, char, String)?;
            Ok(PwListEntry {
                char_params: (char_param1, char_param2),
                limited_char,
                chars: pw_str.chars().collect(),
            })
        })
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day02 {};
        let input = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
        assert_eq!(d.star1(input), "2");
        assert_eq!(d.star2(input), "1");
    }
}
