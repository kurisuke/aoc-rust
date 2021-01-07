use common::day::Day;
use util::hex::HexCoord;

pub struct Day11 {}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let mut pos = HexCoord(0, 0);
        for dir_str in input.trim().split(',') {
            pos += HexCoord::direction_flat(dir_str).unwrap();
        }
        format!("{}", pos.dist(&HexCoord(0, 0)))
    }

    fn star2(&self, input: &str) -> String {
        let mut pos = HexCoord(0, 0);
        let mut max_dist = std::u32::MIN;
        for dir_str in input.trim().split(',') {
            pos += HexCoord::direction_flat(dir_str).unwrap();
            max_dist = max_dist.max(pos.dist(&HexCoord(0, 0)));
        }
        format!("{}", max_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day11 {};
        assert_eq!(d.star1("ne,ne,ne"), "3");
        assert_eq!(d.star1("ne,ne,sw,sw"), "0");
        assert_eq!(d.star1("ne,ne,s,s"), "2");
        assert_eq!(d.star1("se,sw,se,sw,sw"), "3");
    }
}
