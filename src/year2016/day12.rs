use crate::day::Day;
use crate::util::assembunny::{Computer, RegId};

pub struct Day12 {}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let mut computer = Computer::new(input);
        computer.exec();
        format!("{}", computer.get_reg(RegId::A))
    }

    fn star2(&self, input: &str) -> String {
        let mut computer = Computer::new(input);
        computer.set_reg(RegId::C, 1);
        computer.exec();
        format!("{}", computer.get_reg(RegId::A))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day12 {};
        let input = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;
        assert_eq!(d.star1(input), "42");
    }
}
