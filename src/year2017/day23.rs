use crate::day::Day;

pub struct Day23 {}

#[derive(Copy, Clone, Debug)]
enum Val {
    Imm(i64),
    Reg(char),
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(Val, Val),
}

struct Computer {
    register: [i64; 8],
    program: Vec<Op>,
    pc: i64,
    mul_count: usize,
}

impl Computer {
    fn new(program: Vec<Op>) -> Computer {
        Computer {
            register: [0; 8],
            program,
            pc: 0,
            mul_count: 0,
        }
    }

    fn exec(&mut self) {
        while self.pc >= 0 && self.pc < self.program.len() as i64 {
            match self.program[self.pc as usize] {
                Op::Set(r, v) => {
                    self.set_reg(r, self.eval(v));
                    self.pc += 1;
                }
                Op::Sub(r, v) => {
                    let a = self.get_reg(r);
                    let b = self.eval(v);
                    self.set_reg(r, a - b);
                    self.pc += 1;
                }
                Op::Mul(r, v) => {
                    let a = self.get_reg(r);
                    let b = self.eval(v);
                    self.set_reg(r, a * b);
                    self.mul_count += 1;
                    self.pc += 1;
                }
                Op::Jnz(x, y) => {
                    if self.eval(x) != 0 {
                        self.pc += self.eval(y);
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    fn set_reg(&mut self, reg: char, v: i64) {
        self.register[(reg as u8 - b'a') as usize] = v;
    }

    fn get_reg(&self, reg: char) -> i64 {
        self.register[(reg as u8 - b'a') as usize]
    }

    fn eval(&self, v: Val) -> i64 {
        match v {
            Val::Imm(x) => x,
            Val::Reg(c) => self.get_reg(c),
        }
    }

    fn get_mul_count(&self) -> usize {
        self.mul_count
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let w: Vec<_> = line.split_whitespace().collect();
            match w[0] {
                "set" => Op::Set(w[1].chars().next().unwrap(), parse_val(w[2])),
                "sub" => Op::Sub(w[1].chars().next().unwrap(), parse_val(w[2])),
                "mul" => Op::Mul(w[1].chars().next().unwrap(), parse_val(w[2])),
                "jnz" => Op::Jnz(parse_val(w[1]), parse_val(w[2])),
                _ => {
                    panic!("Unknown instruction: {}", w[0]);
                }
            }
        })
        .collect()
}

fn parse_val(s: &str) -> Val {
    match s.parse::<i64>() {
        Ok(x) => Val::Imm(x),
        Err(_) => Val::Reg(s.chars().next().unwrap()),
    }
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer::new(program);
        computer.exec();
        format!("{}", computer.get_mul_count())
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
        let d = Day23 {};
        assert_eq!(d.star1(""), "not implemented");
    }
}
