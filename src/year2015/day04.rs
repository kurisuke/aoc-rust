use crate::day::Day;

pub struct Day04 {}

fn find_hash_with_start(input: &str, start: &str) -> usize {
    let mut i = 1;
    loop {
        let md5_input = format!("{}{}", input, i);
        let digest = md5::compute(md5_input);
        if format!("{:32x}", digest).starts_with(start) {
            break;
        }
        i += 1
    }
    i
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        format!("{}", find_hash_with_start(input, &"00000"))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", find_hash_with_start(input, &"000000"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day04 {};
        assert_eq!(d.star1("abcdef"), "609043");
        assert_eq!(d.star1("pqrstuv"), "1048970");
    }
}
