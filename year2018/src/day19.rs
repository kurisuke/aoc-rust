use common::day::Day;

pub struct Day19 {}

struct Instruction {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

type Registers = [usize; 6];

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

struct State {
    pub program: Vec<Instruction>,
    pub reg: Registers,
    pub ip: usize,
    pub ip_reg: usize,
}

impl State {
    fn run(&mut self) {
        while self.ip < self.program.len() {
            self.reg[self.ip_reg] = self.ip;

            let instr = &self.program[self.ip];
            self.reg = exec_op(instr, &self.reg).unwrap();

            self.ip = self.reg[self.ip_reg];
            self.ip += 1;
        }
    }

    fn run_pt2(&mut self, max_cycles: usize) -> usize {
        let mut cycles = 0;
        let mut max_reg = 0;

        while self.ip < self.program.len() && cycles < max_cycles {
            self.reg[self.ip_reg] = self.ip;

            let instr = &self.program[self.ip];
            self.reg = exec_op(instr, &self.reg).unwrap();

            self.ip = self.reg[self.ip_reg];
            self.ip += 1;

            max_reg = max_reg.max((0..5).map(|i| self.reg[i]).max().unwrap());
            cycles += 1;
        }

        max_reg
    }
}

fn exec_op(instr: &Instruction, regs_in: &Registers) -> Option<Registers> {
    let mut regs_out = *regs_in;
    match instr.op {
        Op::Addr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] + regs_in[instr.b];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Addi => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] + instr.b;
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Mulr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] * regs_in[instr.b];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Muli => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] * instr.b;
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Banr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] & regs_in[instr.b];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Bani => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] & instr.b;
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Borr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] | regs_in[instr.b];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Bori => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a] | instr.b;
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Setr => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = regs_in[instr.a];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Seti => {
            if instr.c < 6 {
                regs_out[instr.c] = instr.a;
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtir => {
            if instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = if instr.a > regs_in[instr.b] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtri => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = if regs_in[instr.a] > instr.b { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtrr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = if regs_in[instr.a] > regs_in[instr.b] {
                    1
                } else {
                    0
                };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Eqir => {
            if instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = if instr.a == regs_in[instr.b] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Eqri => {
            if instr.a < 6 && instr.c < 6 {
                regs_out[instr.c] = if regs_in[instr.a] == instr.b { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Eqrr => {
            if instr.a < 6 && instr.b < 6 && instr.c < 6 {
                regs_out[instr.c] = if regs_in[instr.a] == regs_in[instr.b] {
                    1
                } else {
                    0
                };
                Some(regs_out)
            } else {
                None
            }
        }
    }
}

fn parse_input(input: &str) -> State {
    let ip_reg = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let program: Vec<_> = input
        .lines()
        .skip(1)
        .map(|line| {
            let mut it = line.split_whitespace();
            let op_str = it.next().unwrap();
            let op = match op_str {
                "addr" => Op::Addr,
                "addi" => Op::Addi,
                "mulr" => Op::Mulr,
                "muli" => Op::Muli,
                "banr" => Op::Banr,
                "bani" => Op::Bani,
                "borr" => Op::Borr,
                "bori" => Op::Bori,
                "setr" => Op::Setr,
                "seti" => Op::Seti,
                "gtir" => Op::Gtir,
                "gtri" => Op::Gtri,
                "gtrr" => Op::Gtrr,
                "eqir" => Op::Eqir,
                "eqri" => Op::Eqri,
                "eqrr" => Op::Eqrr,
                _ => {
                    panic!("Unknown op: {}", op_str);
                }
            };
            let a = it.next().unwrap().parse::<usize>().unwrap();
            let b = it.next().unwrap().parse::<usize>().unwrap();
            let c = it.next().unwrap().parse::<usize>().unwrap();
            Instruction { op, a, b, c }
        })
        .collect();
    State {
        program,
        reg: [0, 0, 0, 0, 0, 0],
        ip: 0,
        ip_reg,
    }
}

fn factors(n: usize) -> Vec<usize> {
    let mut fs = vec![];
    let upper = (f64::from(n as i32).sqrt() + 1.0) as usize;
    for i in 1..=upper {
        if n % i == 0 {
            fs.push(i);
            if i * i != n {
                fs.push(n / i);
            }
        }
    }
    fs
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let mut state = parse_input(input);
        state.run();
        format!("{}", state.reg[0])
    }

    fn star2(&self, input: &str) -> String {
        let mut state = parse_input(input);
        state.reg[0] = 1;
        let max_reg = state.run_pt2(30);
        let sum_factors = factors(max_reg).into_iter().sum::<usize>();
        format!("{}", sum_factors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day19 {};

        let input = r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#;

        assert_eq!(d.star1(input), "6");
    }
}
