use common::day::Day;

use util::intcode::{IntSize, Intcode, RunState};

pub struct Day02 {}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let mut init: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<IntSize>().unwrap())
            .collect();
        init[1] = 12;
        init[2] = 2;
        let mut intcode = Intcode::new(&init);
        intcode.run();
        format!("{}", intcode.mem_at(0))
    }

    fn star2(&self, input: &str) -> String {
        let mut init: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<IntSize>().unwrap())
            .collect();
        let mut intcode = Intcode::new(&init);

        const TARGET: IntSize = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                init[1] = noun as IntSize;
                init[2] = verb as IntSize;
                intcode.reset(&init);
                intcode.run();
                if intcode.state == RunState::Halted && intcode.mem_at(0) == TARGET {
                    return format!("{}", 100 * noun + verb);
                }
            }
        }
        String::from("not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let mut intcode = Intcode::new_from_str("1,0,0,0,99");
        intcode.run();
        assert_eq!(intcode.mem_at(0), 2);

        let mut intcode = Intcode::new_from_str("2,3,0,3,99");
        intcode.run();
        assert_eq!(intcode.mem_at(3), 6);

        let mut intcode = Intcode::new_from_str("2,4,4,5,99,0");
        intcode.run();
        assert_eq!(intcode.mem_at(5), 9801);

        let mut intcode = Intcode::new_from_str("1,1,1,4,99,5,6,0,99");
        intcode.run();
        assert_eq!(intcode.mem_at(0), 30);
        assert_eq!(intcode.mem_at(4), 2);
    }
}
