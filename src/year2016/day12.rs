use crate::day::Day;

pub struct Day12 {}

#[derive(Copy, Clone)]
enum RegId {
    A,
    B,
    C,
    D,
}

enum Op {
    CpyI(i64, RegId),
    CpyR(RegId, RegId),
    Inc(RegId),
    Dec(RegId),
    JnzI(i64, isize),
    JnzR(RegId, isize),
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    pc: isize,
    program: Vec<Op>,
}

impl Computer {
    fn new(program: Vec<Op>) -> Computer {
        Computer {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            program,
        }
    }

    fn exec(&mut self) {
        while self.pc >= 0 && self.pc < self.program.len() as isize {
            match self.program[self.pc as usize] {
                Op::CpyI(x, y) => {
                    self.set_reg(y, x);
                    self.pc += 1;
                }
                Op::CpyR(x, y) => {
                    self.set_reg(y, self.get_reg(x));
                    self.pc += 1;
                }
                Op::Inc(x) => {
                    self.set_reg(x, self.get_reg(x) + 1);
                    self.pc += 1;
                }
                Op::Dec(x) => {
                    self.set_reg(x, self.get_reg(x) - 1);
                    self.pc += 1;
                }
                Op::JnzI(x, y) => {
                    if x != 0 {
                        self.pc += y;
                    } else {
                        self.pc += 1;
                    }
                }
                Op::JnzR(x, y) => {
                    if self.get_reg(x) != 0 {
                        self.pc += y;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    fn get_reg(&self, id: RegId) -> i64 {
        match id {
            RegId::A => self.a,
            RegId::B => self.b,
            RegId::C => self.c,
            RegId::D => self.d,
        }
    }

    fn set_reg(&mut self, id: RegId, v: i64) {
        match id {
            RegId::A => {
                self.a = v;
            }
            RegId::B => {
                self.b = v;
            }
            RegId::C => {
                self.c = v;
            }
            RegId::D => {
                self.d = v;
            }
        }
    }
}

fn to_regid(s: &str) -> Option<RegId> {
    let c = s.chars().next().unwrap();
    match c {
        'a' => Some(RegId::A),
        'b' => Some(RegId::B),
        'c' => Some(RegId::C),
        'd' => Some(RegId::D),
        _ => None,
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            match words[0] {
                "cpy" => match words[1].parse::<i64>() {
                    Ok(x) => Some(Op::CpyI(x, to_regid(words[2]).unwrap())),
                    Err(_) => Some(Op::CpyR(
                        to_regid(words[1]).unwrap(),
                        to_regid(words[2]).unwrap(),
                    )),
                },
                "inc" => Some(Op::Inc(to_regid(words[1]).unwrap())),
                "dec" => Some(Op::Dec(to_regid(words[1]).unwrap())),
                "jnz" => match words[1].parse::<i64>() {
                    Ok(x) => Some(Op::JnzI(x, words[2].parse::<isize>().unwrap())),
                    Err(_) => Some(Op::JnzR(
                        to_regid(words[1]).unwrap(),
                        words[2].parse::<isize>().unwrap(),
                    )),
                },
                _ => None,
            }
        })
        .filter_map(|x| x)
        .collect()
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer::new(program);
        computer.exec();
        format!("{}", computer.get_reg(RegId::A))
    }

    fn star2(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer::new(program);
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
