use crate::day::Day;

pub struct Day23 {}

enum Reg {
    RegA,
    RegB,
}

enum Op {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

#[derive(PartialEq)]
enum State {
    Running,
    Finished,
}

struct Computer {
    reg_a: usize,
    reg_b: usize,
    program: Vec<Op>,
    ic: isize,
}

impl Computer {
    fn new(program: Vec<Op>) -> Computer {
        Computer {
            reg_a: 0,
            reg_b: 0,
            program,
            ic: 0,
        }
    }

    fn next(&mut self) -> State {
        if self.ic < 0 || self.ic as usize >= self.program.len() {
            State::Finished
        } else {
            match &self.program[self.ic as usize] {
                Op::Hlf(r) => {
                    match r {
                        Reg::RegA => self.reg_a /= 2,
                        Reg::RegB => self.reg_b /= 2,
                    }
                    self.ic += 1;
                }
                Op::Tpl(r) => {
                    match r {
                        Reg::RegA => self.reg_a *= 3,
                        Reg::RegB => self.reg_b *= 3,
                    }
                    self.ic += 1;
                }
                Op::Inc(r) => {
                    match r {
                        Reg::RegA => {
                            self.reg_a += 1;
                        }
                        Reg::RegB => {
                            self.reg_b += 1;
                        }
                    }
                    self.ic += 1;
                }
                Op::Jmp(off) => {
                    self.ic += off;
                }
                Op::Jie(r, off) => {
                    let r_val = match r {
                        Reg::RegA => self.reg_a,
                        Reg::RegB => self.reg_b,
                    };
                    if r_val % 2 == 0 {
                        self.ic += off;
                    } else {
                        self.ic += 1;
                    }
                }
                Op::Jio(r, off) => {
                    let r_val = match r {
                        Reg::RegA => self.reg_a,
                        Reg::RegB => self.reg_b,
                    };
                    if r_val == 1 {
                        self.ic += off;
                    } else {
                        self.ic += 1;
                    }
                }
            }
            State::Running
        }
    }

    fn run(&mut self) {
        while self.next() == State::Running {}
    }

    fn reg(self, r: Reg) -> usize {
        match r {
            Reg::RegA => self.reg_a,
            Reg::RegB => self.reg_b,
        }
    }

    fn set_reg(&mut self, r: Reg, v: usize) {
        match r {
            Reg::RegA => {
                self.reg_a = v;
            }
            Reg::RegB => {
                self.reg_b = v;
            }
        }
    }
}

fn map_reg(c: char) -> Reg {
    match c {
        'a' => Reg::RegA,
        'b' => Reg::RegB,
        _ => {
            panic!("Invalid register: {}", c);
        }
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            match parts[0] {
                "hlf" => Op::Hlf(map_reg(parts[1].chars().next().unwrap())),
                "tpl" => Op::Tpl(map_reg(parts[1].chars().next().unwrap())),
                "inc" => Op::Inc(map_reg(parts[1].chars().next().unwrap())),
                "jmp" => Op::Jmp(parts[1].parse::<isize>().unwrap()),
                "jie" => Op::Jie(
                    map_reg(parts[1].chars().next().unwrap()),
                    parts[2].parse::<isize>().unwrap(),
                ),
                "jio" => Op::Jio(
                    map_reg(parts[1].chars().next().unwrap()),
                    parts[2].parse::<isize>().unwrap(),
                ),
                _ => {
                    panic!("Invalid instruction: {}", parts[0]);
                }
            }
        })
        .collect()
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer::new(program);
        computer.run();
        format!("{}", computer.reg(Reg::RegB))
    }

    fn star2(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut computer = Computer::new(program);
        computer.set_reg(Reg::RegA, 1);
        computer.run();
        format!("{}", computer.reg(Reg::RegB))
    }
}
