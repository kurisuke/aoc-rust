use common::day::Day;
use util::wrist::Wrist;

pub struct Day21 {}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let mut wrist = Wrist::new_from_str(input);
        format!("{}", wrist.run_d21_s1().unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let mut wrist = Wrist::new_from_str(input);
        format!("{}", wrist.run_d21_s2().unwrap())
    }
}
