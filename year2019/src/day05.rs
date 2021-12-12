use common::day::Day;

use util::intcode::{IntSize, Intcode, RunState};

pub struct Day05 {}

fn run_program(input: &str, program_id: IntSize) -> IntSize {
    let mut intcode = Intcode::new_from_str(input);
    intcode.write_inp(program_id);
    intcode.run();
    assert_eq!(intcode.state, RunState::Halted);
    let outputs = intcode.read_outp_all();
    let (diag_code, test_results) = outputs.split_last().unwrap();
    assert!(test_results.iter().all(|&x| x == 0));
    *diag_code
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        format!("{}", run_program(input, 1))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", run_program(input, 5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        let init = r#"3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"#;

        let mut intcode = Intcode::new_from_str(init);
        intcode.write_inp(7);
        intcode.run();
        assert_eq!(intcode.state, RunState::Halted);
        assert_eq!(intcode.read_outp(), Some(999));

        intcode.reset_from_str(init);
        intcode.write_inp(8);
        intcode.run();
        assert_eq!(intcode.state, RunState::Halted);
        assert_eq!(intcode.read_outp(), Some(1000));

        intcode.reset_from_str(init);
        intcode.write_inp(9);
        intcode.run();
        assert_eq!(intcode.state, RunState::Halted);
        assert_eq!(intcode.read_outp(), Some(1001));
    }
}
