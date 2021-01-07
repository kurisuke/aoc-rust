use common::day::Day;
use util::knothash::{knot, KnotHash};

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let lengths: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut v: Vec<_> = (0..=255).into_iter().collect();
        knot(&mut v, &lengths, 0, 0);
        format!("{}", v[0] as usize * v[1] as usize)
    }

    fn star2(&self, input: &str) -> String {
        let hash = KnotHash::from(input.trim());
        format!("{}", hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let mut v: Vec<_> = (0..=4).into_iter().collect();
        let lengths = vec![3, 4, 1, 5];
        knot(&mut v, &lengths, 0, 0);
        assert_eq!(v[0] * v[1], 12);
    }

    #[test]
    fn star2() {
        let d = Day10 {};
        assert_eq!(d.star2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(d.star2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(d.star2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(d.star2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
