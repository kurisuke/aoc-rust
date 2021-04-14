use common::day::Day;

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, _input: &str) -> String {
        String::from("not implemented")
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
        let d = Day10 {};
        assert_eq!(d.star1(""), "not implemented");
    }
}
