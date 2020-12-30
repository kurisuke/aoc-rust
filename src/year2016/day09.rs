use crate::day::Day;

pub struct Day09 {}

enum ScanMode {
    Normal,
    ScanMarker,
    ScanRepeat(usize, usize),
}

fn decompress(s: &str) -> String {
    let mut mode = ScanMode::Normal;

    let mut out = String::new();
    let mut buffer = String::new();
    let mut num_chars_tmp: usize = 0;

    for c in s.chars() {
        match mode {
            ScanMode::Normal => {
                if c == '(' {
                    mode = ScanMode::ScanMarker;
                } else {
                    out.push(c);
                }
            }
            ScanMode::ScanMarker => {
                if c == 'x' {
                    // first parameter is done
                    num_chars_tmp = buffer.parse::<usize>().unwrap();
                    buffer.clear();
                } else if c == ')' {
                    let times = buffer.parse::<usize>().unwrap();
                    buffer.clear();
                    mode = ScanMode::ScanRepeat(num_chars_tmp, times);
                } else {
                    buffer.push(c);
                }
            }
            ScanMode::ScanRepeat(num_chars, times) => {
                buffer.push(c);
                if buffer.len() == num_chars {
                    for _ in 0..times {
                        out += &buffer;
                    }
                    buffer.clear();
                    mode = ScanMode::Normal;
                }
            }
        }
    }
    out
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let total = input
            .lines()
            .map(|line| decompress(line).len())
            .sum::<usize>();
        format!("{}", total)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day09 {};
        assert_eq!(d.star1("ADVENT"), "6");
        assert_eq!(d.star1("A(1x5)BC"), "7");
        assert_eq!(d.star1("(3x3)XYZ"), "9");
        assert_eq!(d.star1("A(2x2)BCD(2x2)EFG"), "11");
        assert_eq!(d.star1("(6x1)(1x3)A"), "6");
        assert_eq!(d.star1("X(8x2)(3x3)ABCY"), "18");
    }
}
