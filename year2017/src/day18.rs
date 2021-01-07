use common::day::Day;
use std::collections::VecDeque;

pub struct Day18 {}

#[derive(Copy, Clone, Debug)]
enum Val {
    Imm(i64),
    Reg(char),
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(Val, Val),
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Stopped,
    Blocked,
}

struct Computer1 {
    register: [i64; 26],
    last_played: Option<i64>,
    program: Vec<Op>,
    pc: i64,
}

struct Computer2 {
    register: [i64; 26],
    program: Vec<Op>,
    pc: i64,
    state: State,
    input: VecDeque<i64>,
    output: Vec<i64>,
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
                Op::Rcv(r) => {
                    if self.get_reg(r) != 0 {
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

    fn get_last_played(&self) -> Option<i64> {
        self.last_played
    }
}

impl Computer2 {
    fn new(program: Vec<Op>) -> Computer2 {
        Computer2 {
            register: [0; 26],
            program,
            pc: 0,
            state: State::Stopped,
            input: VecDeque::new(),
            output: vec![],
        }
    }

    fn exec(&mut self) {
        while self.pc >= 0 && self.pc < self.program.len() as i64 {
            match self.program[self.pc as usize] {
                Op::Snd(v) => {
                    self.output.push(self.eval(v));
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
                Op::Rcv(r) => {
                    if self.input.is_empty() {
                        self.state = State::Blocked;
                        return;
                    } else {
                        let v = self.input.pop_front().unwrap();
                        self.set_reg(r, v);
                        self.pc += 1;
                    }
                }
            }
        }
        self.state = State::Stopped;
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

    fn write(&mut self, s: &[i64]) {
        self.input.extend(s);
    }

    fn read(&mut self) -> Vec<i64> {
        let tmp = self.output.clone();
        self.output.clear();
        tmp
    }

    fn get_state(&self) -> State {
        self.state
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let w: Vec<_> = line.split_whitespace().collect();
            match w[0] {
                "snd" => Op::Snd(parse_val(w[1])),
                "set" => Op::Set(w[1].chars().next().unwrap(), parse_val(w[2])),
                "add" => Op::Add(w[1].chars().next().unwrap(), parse_val(w[2])),
                "mul" => Op::Mul(w[1].chars().next().unwrap(), parse_val(w[2])),
                "mod" => Op::Mod(w[1].chars().next().unwrap(), parse_val(w[2])),
                "rcv" => Op::Rcv(w[1].chars().next().unwrap()),
                "jgz" => Op::Jgz(parse_val(w[1]), parse_val(w[2])),
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

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer1::new(program);
        computer.exec(true);
        format!("{}", computer.get_last_played().unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut comp_a = Computer2::new(program.clone());
        comp_a.set_reg('p', 0);
        let mut comp_b = Computer2::new(program);
        comp_b.set_reg('p', 1);

        let mut num_read_from_b = 0;
        loop {
            comp_a.exec();
            comp_b.exec();

            // both programs terminated
            if comp_a.get_state() == State::Stopped && comp_b.get_state() == State::Stopped {
                break;
            }

            let read_from_a = comp_a.read();
            comp_b.write(&read_from_a);

            let read_from_b = comp_b.read();
            comp_a.write(&read_from_b);

            // deadlock detection
            if read_from_a.is_empty() && read_from_b.is_empty() {
                break;
            }

            num_read_from_b += read_from_b.len();
        }
        format!("{}", num_read_from_b)
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
