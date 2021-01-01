use crate::day::Day;
use crate::util::assembunny::{Computer, RegId};

pub struct Day25 {}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let mut i = 0;
        let target = "01".repeat(50);
        loop {
            let mut computer = Computer::new(input);
            computer.set_reg(RegId::A, i);
            let output = computer.exec(Some(100), Some(1_000_000));
            if output == target {
                break;
            }
            i += 1;
        }
        format!("{}", i)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}
