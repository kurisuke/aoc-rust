use common::day::Day;
use md5::{Digest, Md5};

pub struct Day05 {}

fn password_part1(prefix: &str) -> String {
    let mut i = 0;
    let mut pw = String::new();
    let mut hasher = Md5::new();
    while pw.len() < 8 {
        hasher.update(prefix.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize_reset();
        let start = result[0] as usize + result[1] as usize + (result[2] >> 4) as usize;
        if start == 0 {
            pw += &format!("{:x}", result[2] & 0x0f);
        }
        i += 1;
    }
    pw
}

fn password_part2(prefix: &str) -> String {
    let mut i = 0;
    let mut pw: [Option<char>; 8] = [None; 8];
    let mut hasher = Md5::new();
    loop {
        hasher.update(prefix.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize_reset();
        let start = result[0] as usize + result[1] as usize + (result[2] >> 4) as usize;
        if start == 0 {
            let pos = (result[2] & 0xf) as usize;
            if pos < 8 && pw[pos].is_none() {
                pw[pos] = Some(format!("{:x}", result[3] >> 4).chars().next().unwrap());
                if pw.iter().all(|x| x.is_some()) {
                    break;
                }
            }
        }
        i += 1;
    }
    pw.iter().filter_map(|x| *x).collect()
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        password_part1(input)
    }

    fn star2(&self, input: &str) -> String {
        password_part2(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn ex1() {
        let d = Day05 {};
        assert_eq!(d.star1("abc"), "18f47a30");
        assert_eq!(d.star2("abc"), "05ace8e3");
    }
}
