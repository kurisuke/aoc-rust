use common::day::Day;

pub struct Day01 {}

fn sum_equal(digits: &[char], off: usize) -> u32 {
    digits
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if *c == digits[(i + off) % digits.len()] {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .flatten()
        .sum::<u32>()
}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let digits: Vec<_> = input.trim().chars().collect();
        let captcha = sum_equal(&digits, 1);
        format!("{}", captcha)
    }

    fn star2(&self, input: &str) -> String {
        let digits: Vec<_> = input.trim().chars().collect();
        let captcha = sum_equal(&digits, digits.len() / 2);
        format!("{}", captcha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day01 {};
        assert_eq!(d.star1("1122"), "3");
        assert_eq!(d.star1("1111"), "4");
        assert_eq!(d.star1("1234"), "0");
        assert_eq!(d.star1("91212129"), "9");
    }

    #[test]
    fn star2() {
        let d = Day01 {};
        assert_eq!(d.star2("1212"), "6");
        assert_eq!(d.star2("1221"), "0");
        assert_eq!(d.star2("123425"), "4");
        assert_eq!(d.star2("123123"), "12");
        assert_eq!(d.star2("12131415"), "4");
    }
}
