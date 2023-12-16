use common::day::Day;

pub struct Day15 {}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let input = input.replace('\n', "");
        input.split(',').map(hash).sum::<usize>().to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h += (c as u8) as usize;
        h *= 17;
        h %= 256;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn ex1() {
        let d = Day15 {};
        assert_eq!(d.star1(INPUT), "1320");
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
