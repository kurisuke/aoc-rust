use common::day::Day;
use itertools::Itertools;
use util::intcode::{IntSize, Intcode, RunState};

pub struct Day07 {}

struct AmpSetup<'a> {
    amps: Vec<Intcode>,
    connections: Vec<Option<usize>>,
    program: &'a str,
}

impl AmpSetup<'_> {
    fn new(num_amps: usize, connections: Vec<Option<usize>>, program: &str) -> AmpSetup {
        AmpSetup {
            amps: vec![Intcode::new_from_str(program); num_amps],
            connections,
            program,
        }
    }

    fn run(&mut self, input_vals: &[IntSize]) -> IntSize {
        for (i, input_val) in input_vals.iter().enumerate() {
            self.amps[i].reset_from_str(self.program);
            self.amps[i].write_inp(*input_val);
        }
        self.amps[0].write_inp(0);

        let mut cur = 0;
        while self.amps[cur].state != RunState::Halted {
            self.amps[cur].run();
            if let Some(next) = self.connections[cur] {
                if self.amps[next].state != RunState::Halted {
                    self.propagate(cur, next);
                    cur = next;
                }
            }
        }
        *self.amps[cur].read_outp_all().last().unwrap()
    }

    fn propagate(&mut self, out_amp: usize, in_amp: usize) {
        while let Some(v) = self.amps[out_amp].read_outp() {
            self.amps[in_amp].write_inp(v);
        }
    }
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let mut setup = AmpSetup::new(5, vec![Some(1), Some(2), Some(3), Some(4), None], input);
        let max_output = (0..5)
            .permutations(5)
            .map(|vals| setup.run(&vals))
            .max()
            .unwrap();
        format!("{}", max_output)
    }

    fn star2(&self, input: &str) -> String {
        let mut setup = AmpSetup::new(5, vec![Some(1), Some(2), Some(3), Some(4), Some(0)], input);
        let max_output = (5..10)
            .permutations(5)
            .map(|vals| setup.run(&vals))
            .max()
            .unwrap();
        format!("{}", max_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let d = Day07 {};
        assert_eq!(
            d.star1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            "43210"
        );
        assert_eq!(
            d.star1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            "54321"
        );
        assert_eq!(d.star1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), "65210");
    }

    #[test]
    fn test_star2() {
        let d = Day07 {};
        assert_eq!(d.star2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), "139629729");
        assert_eq!(d.star2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), "18216");
    }
}
