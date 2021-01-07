use common::day::Day;

pub struct Day09 {}

enum ScanMode {
    Normal,
    ScanMarker,
    ScanRepeat(usize, usize),
}

fn decompressed_len(s: &str, recurse: bool) -> usize {
    let mut mode = ScanMode::Normal;

    let mut out = 0;
    let mut buffer = String::new();
    let mut num_chars_tmp: usize = 0;

    for c in s.chars() {
        match mode {
            ScanMode::Normal => {
                if c == '(' {
                    mode = ScanMode::ScanMarker;
                } else {
                    out += 1;
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
                    if recurse {
                        out += decompressed_len(&buffer, true) * times;
                    } else {
                        out += buffer.len() * times;
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
            .map(|line| decompressed_len(line, false))
            .sum::<usize>();
        format!("{}", total)
    }

    fn star2(&self, input: &str) -> String {
        let total = input
            .lines()
            .map(|line| decompressed_len(line, true))
            .sum::<usize>();
        format!("{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day09 {};
        assert_eq!(d.star1("ADVENT"), "6");
        assert_eq!(d.star1("A(1x5)BC"), "7");
        assert_eq!(d.star1("(3x3)XYZ"), "9");
        assert_eq!(d.star1("A(2x2)BCD(2x2)EFG"), "11");
        assert_eq!(d.star1("(6x1)(1x3)A"), "6");
        assert_eq!(d.star1("X(8x2)(3x3)ABCY"), "18");
    }

    #[test]
    fn star2() {
        let d = Day09 {};
        assert_eq!(d.star2("(3x3)XYZ"), "9");
        assert_eq!(d.star2("X(8x2)(3x3)ABCY"), "20");
        assert_eq!(d.star2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), "241920");
        assert_eq!(
            d.star2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            "445"
        );
    }
}
