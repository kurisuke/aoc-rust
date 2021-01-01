use crate::day::Day;
use crate::util::assembunny::{Computer, RegId};

pub struct Day23 {}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let mut computer = Computer::new(input);
        computer.set_reg(RegId::A, 7);
        computer.exec();
        format!("{}", computer.get_reg(RegId::A))
    }

    fn star2(&self, input: &str) -> String {
        let mut computer = Computer::new(input);
        computer.set_reg(RegId::A, 12);
        computer.exec();
        format!("{}", computer.get_reg(RegId::A))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day23 {};
        let input = r#"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a"#;
        assert_eq!(d.star1(input), "3");
    }
}
