use crate::day::Day;

pub struct Day18 {}

#[derive(Copy,Clone,Debug)]
enum Val {
    Imm(i64),
    Reg(char),
}

#[derive(Copy,Clone, Debug)]
enum Op {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(Val),
    Jgz(Val, Val),
}

struct Computer1 {
    register: [i64; 26],
    last_played: Option<i64>,
    program: Vec<Op>,
    pc: i64,
}

impl Computer1 {
    fn new(program: Vec<Op>) -> Computer1 {
        Computer1 {
            register: [0; 26],
            last_played: None,
            program,
            pc: 0,
        }
    }

    fn exec(&mut self, stop_on_rcv: bool) {
        while self.pc >= 0 && self.pc < self.program.len() as i64 {
            match self.program[self.pc as usize] {
                Op::Snd(v) => {
                    self.last_played = Some(self.eval(v));
                    self.pc += 1;
                }
                Op::Set(r, v) => {
                    self.set_reg(r, self.eval(v));
                    self.pc += 1;
                }
                Op::Add(r, v) => {
                    let a = self.get_reg(r);
                    let b = self.eval(v);
                    self.set_reg(r, a + b);
                    self.pc += 1;
                }
                Op::Mul(r, v) => {
                    let a = self.get_reg(r);
                    let b = self.eval(v);
                    self.set_reg(r, a * b);
                    self.pc += 1;
                }
                Op::Mod(r, v) => {
                    let a = self.get_reg(r);
                    let b = self.eval(v);
                    self.set_reg(r, a % b);
                    self.pc += 1;
                }
                Op::Jgz(x, y) => {
                    if self.eval(x) > 0 {
                        self.pc += self.eval(y);
                    } else {
                        self.pc += 1;
                    }
                }
                Op::Rcv(v) => {
                    if self.eval(v) != 0 {
                        if stop_on_rcv {
                            return;
                        }
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }

    fn set_reg(&mut self, reg: char, v: i64) {
        self.register[(reg as u8 - 'a' as u8) as usize] = v;
    }

    fn get_reg(&self, reg: char) -> i64 {
        self.register[(reg as u8 - 'a' as u8) as usize]
    }

    fn eval(&self, v: Val) -> i64 {
        match v {
            Val::Imm(x) => x,
            Val::Reg(c) => self.get_reg(c),
        }
    }

    fn get_last_played(&self) -> Option<i64> {
        self.last_played
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input.lines().map(|line| {
        let w: Vec<_> = line.split_whitespace().collect();
        match w[0] {
            "snd" => Op::Snd(parse_val(w[1])),
            "set" => Op::Set(w[1].chars().next().unwrap(), parse_val(w[2])),
            "add" => Op::Add(w[1].chars().next().unwrap(), parse_val(w[2])),
            "mul" => Op::Mul(w[1].chars().next().unwrap(), parse_val(w[2])),
            "mod" => Op::Mod(w[1].chars().next().unwrap(), parse_val(w[2])),
            "rcv" => Op::Rcv(parse_val(w[1])),
            "jgz" => Op::Jgz(parse_val(w[1]), parse_val(w[2])),
            _ => { panic!("Unknown instruction: {}", w[0]); }
        }
    }).collect()
}

fn parse_val(s: &str) -> Val {
    match s.parse::<i64>() {
        Ok(x) => Val::Imm(x),
        Err(_) => Val::Reg(s.chars().next().unwrap()),
    }
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer1::new(program);
        computer.exec(true);
        format!("{}", computer.get_last_played().unwrap())
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
        let d = Day18 {};
        let input = r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#;
        assert_eq!(d.star1(input), "4");
    }
}
