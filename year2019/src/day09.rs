use common::day::Day;
use util::intcode::Intcode;

pub struct Day09 {}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        intcode.write_inp(1);
        intcode.run();
        let outp = intcode.read_outp_all();
        assert_eq!(outp.len(), 1);
        format!("{}", outp[0])
    }

    fn star2(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        intcode.write_inp(2);
        intcode.run();
        format!("{}", intcode.read_outp().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut intcode =
            Intcode::new_from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        intcode.run();
        let out = intcode.read_outp_all();
        assert_eq!(
            out,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn ex2() {
        let mut intcode = Intcode::new_from_str("1102,34915192,34915192,7,4,7,99,0");
        intcode.run();
        let out = intcode.read_outp().unwrap();
        assert_eq!(format!("{}", out).len(), 16);
    }

    #[test]
    fn ex3() {
        let mut intcode = Intcode::new_from_str("104,1125899906842624,99");
        intcode.run();
        let out = intcode.read_outp().unwrap();
        assert_eq!(out, 1125899906842624);
    }
}
