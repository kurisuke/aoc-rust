use common::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let end_floor = input.chars().filter(|&c| c == '(').count() as i32
            - input.chars().filter(|&c| c == ')').count() as i32;
        format!("{}", end_floor)
    }

    fn star2(&self, input: &str) -> String {
        let mut cur_floor = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => {
                    cur_floor += 1;
                }
                ')' => {
                    cur_floor -= 1;
                }
                _ => {}
            }
            if cur_floor < 0 {
                return format!("{}", i + 1);
            }
        }
        String::from("err")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let d = Day01 {};
        assert_eq!(d.star1("(())"), "0");
        assert_eq!(d.star1("()()"), "0");
        assert_eq!(d.star1("((("), "3");
        assert_eq!(d.star1("(()(()("), "3");
        assert_eq!(d.star1("))((((("), "3");
        assert_eq!(d.star1("())"), "-1");
        assert_eq!(d.star1("))("), "-1");
        assert_eq!(d.star1(")))"), "-3");
        assert_eq!(d.star1(")())())"), "-3");
    }

    #[test]
    fn part2() {
        let d = Day01 {};
        assert_eq!(d.star2(")"), "1");
        assert_eq!(d.star2("()())"), "5");
    }
}
