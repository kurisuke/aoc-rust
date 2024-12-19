use std::{cmp::Reverse, collections::BinaryHeap};

use common::day::Day;
use itertools::Itertools;

pub struct Day17 {}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let mut computer = Computer::parse(input);
        computer.run_until_halt();
        computer.output.iter().map(|s| s.to_string()).join(",")
    }

    fn star2(&self, input: &str) -> String {
        let computer_orig = Computer::parse(input);
        search(computer_orig).unwrap().to_string()
    }
}

#[derive(Clone)]
struct Computer {
    program: Vec<usize>,
    pub reg: [usize; 3],
    ip: usize,
    pub output: Vec<usize>,
}

impl Computer {
    #[cfg(test)]
    pub fn new(program: Vec<usize>, reg: [usize; 3]) -> Self {
        Self {
            program,
            reg,
            ip: 0,
            output: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn disassembly(&self) {
        let mut ip = 0;
        while ip < self.program.len() {
            let instr = match self.program[ip] {
                0 => "ADV",
                1 => "BXL",
                2 => "BST",
                3 => "JNZ",
                4 => "BXC",
                5 => "OUT",
                6 => "BDV",
                7 => "CDV",
                _ => unreachable!(),
            };

            let operand = match self.program[ip] {
                0 | 6 | 7 => combo_str(self.program[ip + 1]),
                1 | 3 => self.program[ip + 1].to_string(),
                2 | 5 => format!("{} mod 8", combo_str(self.program[ip + 1])),
                4 => String::new(),
                _ => unreachable!(),
            };

            println!("{:04}: {} {}", ip, instr, operand);
            ip += 2;
        }
    }

    pub fn parse(input: &str) -> Self {
        let secs: Vec<_> = input.split("\n\n").collect();

        let mut reg = [0; 3];
        for (i, line) in secs[0].lines().enumerate() {
            let reg_val = line.split(": ").nth(1).unwrap();
            reg[i] = reg_val.parse().unwrap();
        }

        let program = secs[1].trim().split(": ").nth(1).unwrap();
        let program = program.split(',').map(|n| n.parse().unwrap()).collect();

        Self {
            program,
            reg,
            ip: 0,
            output: vec![],
        }
    }

    pub fn run_until_halt(&mut self) {
        loop {
            if self.step() {
                break;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        match self.program[self.ip] {
            0 => {
                // adv
                self.reg[0] >>= self.combo();
                self.ip += 2;
            }
            1 => {
                // bxl
                self.reg[1] ^= self.literal();
                self.ip += 2;
            }
            2 => {
                // bst
                self.reg[1] = self.combo() % 8;
                self.ip += 2;
            }
            3 => {
                // jnz
                if self.reg[0] == 0 {
                    self.ip += 2;
                } else {
                    self.ip = self.literal();
                }
            }
            4 => {
                // bxc
                self.reg[1] ^= self.reg[2];
                self.ip += 2;
            }
            5 => {
                // out
                self.output.push(self.combo() % 8);
                self.ip += 2;
            }
            6 => {
                // bdv
                self.reg[1] = self.reg[0] >> self.combo();
                self.ip += 2;
            }
            7 => {
                // cdv
                self.reg[2] = self.reg[0] >> self.combo();
                self.ip += 2;
            }
            _ => unreachable!(),
        }

        self.ip + 1 >= self.program.len()
    }

    fn literal(&self) -> usize {
        self.program[self.ip + 1]
    }

    fn combo(&self) -> usize {
        let v = self.program[self.ip + 1];
        match v {
            0..=3 => v,
            4..=6 => self.reg[v - 4],
            _ => unreachable!(),
        }
    }
}

fn combo_str(n: usize) -> String {
    match n {
        0..=3 => n.to_string(),
        4 => "<regA>".to_owned(),
        5 => "<regB>".to_owned(),
        6 => "<regC>".to_owned(),
        _ => unreachable!(),
    }
}

fn search(computer_orig: Computer) -> Option<usize> {
    let mut search_states = BinaryHeap::new();
    search_states.push(Reverse(0));
    while let Some(Reverse(a)) = search_states.pop() {
        for a_add in 0..8 {
            let a_new = a + a_add;
            let mut computer = computer_orig.clone();
            computer.reg[0] = a_new;
            computer.run_until_halt();
            // println!("{}: {:?}", a_new, computer.output);
            if computer.program == computer.output {
                return Some(a_new);
            }

            if computer.program[computer.program.len() - computer.output.len()..] == computer.output
            {
                search_states.push(Reverse(a_new << 3));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    #[test]
    fn example_programs() {
        let mut c = Computer::new(vec![2, 6], [0, 0, 9]);
        c.run_until_halt();
        assert_eq!(c.reg[1], 1);

        let mut c = Computer::new(vec![5, 0, 5, 1, 5, 4], [10, 0, 0]);
        c.run_until_halt();
        assert_eq!(c.output, [0, 1, 2]);

        let mut c = Computer::new(vec![0, 1, 5, 4, 3, 0], [2024, 0, 0]);
        c.run_until_halt();
        assert_eq!(c.output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(c.reg[0], 0);

        let mut c = Computer::new(vec![1, 7], [0, 29, 0]);
        c.run_until_halt();
        assert_eq!(c.reg[1], 26);

        let mut c = Computer::new(vec![4, 0], [0, 2024, 43690]);
        c.run_until_halt();
        assert_eq!(c.reg[1], 44354);
    }

    #[test]
    fn star1() {
        let d = Day17 {};
        assert_eq!(d.star1(INPUT1), "4,6,3,5,6,3,5,2,1,0");
    }
}
