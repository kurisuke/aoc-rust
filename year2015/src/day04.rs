use common::day::Day;
use md5::{Digest, Md5};

pub struct Day04 {}

fn find_hash_star1(input: &str) -> usize {
    let mut i = 1;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize_reset();
        let start = result[0] as usize + result[1] as usize + (result[2] >> 4) as usize;
        if start == 0 {
            break;
        }
        i += 1
    }
    i
}

fn find_hash_star2(input: &str) -> usize {
    let mut i = 1;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize_reset();
        let start = result[0] as usize + result[1] as usize + result[2] as usize;
        if start == 0 {
            break;
        }
        i += 1
    }
    i
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        format!("{}", find_hash_star1(input))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", find_hash_star2(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn ex1() {
        let d = Day04 {};
        assert_eq!(d.star1("abcdef"), "609043");
        assert_eq!(d.star1("pqrstuv"), "1048970");
    }
}
