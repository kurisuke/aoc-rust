use std::collections::{HashMap, VecDeque};

pub type IntSize = i64;

struct IntcodeMem {
    mem: HashMap<IntSize, IntSize>,
}

impl IntcodeMem {
    pub fn new(init: &[IntSize]) -> IntcodeMem {
        let mem = init
            .iter()
            .enumerate()
            .map(|(a, v)| (a as IntSize, *v))
            .collect();
        IntcodeMem { mem }
    }

    pub fn load(&self, a: IntSize) -> IntSize {
        *self.mem.get(&a).unwrap_or(&0)
    }

    pub fn store(&mut self, a: IntSize, v: IntSize) {
        self.mem.insert(a, v);
    }
}

pub struct Intcode {
    pub state: RunState,

    mem: IntcodeMem,
    pc: IntSize,
    inp: VecDeque<IntSize>,
    outp: VecDeque<IntSize>,
    rel_base: IntSize,
}

#[derive(Debug, PartialEq)]
pub enum RunState {
    Halted,
    Running,
    Blocked,
    Error,
}

pub enum AddrMode {
    Pos,
    Imm,
    Rel,
    Invalid(IntSize),
}

impl AddrMode {
    pub fn from(n: usize, op: IntSize) -> AddrMode {
        #[allow(clippy::unnecessary_cast)]
        let mode_i = (op / (10 as IntSize).pow(n as u32 + 2)) % 10;
        match mode_i {
            0 => AddrMode::Pos,
            1 => AddrMode::Imm,
            2 => AddrMode::Rel,
            _ => AddrMode::Invalid(mode_i),
        }
    }
}

impl Intcode {
    pub fn new(init: &[IntSize]) -> Intcode {
        Intcode {
            mem: IntcodeMem::new(init),
            pc: 0,
            inp: VecDeque::new(),
            outp: VecDeque::new(),
            state: RunState::Halted,
            rel_base: 0,
        }
    }

    pub fn new_from_str(input: &str) -> Intcode {
        let init: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<IntSize>().unwrap())
            .collect();
        Self::new(&init)
    }

    pub fn reset(&mut self, init: &[IntSize]) {
        self.mem = IntcodeMem::new(init);
        self.pc = 0;
        self.inp.clear();
        self.outp.clear();
        self.state = RunState::Halted;
        self.rel_base = 0;
    }

    pub fn reset_from_str(&mut self, input: &str) {
        let init: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<IntSize>().unwrap())
            .collect();
        self.reset(&init);
    }

    pub fn write_inp(&mut self, n: IntSize) {
        self.inp.push_back(n);
    }

    pub fn write_inp_ascii(&mut self, s: &str) {
        s.chars().for_each(|c| self.write_inp(c as IntSize));
    }

    pub fn read_outp(&mut self) -> Option<IntSize> {
        self.outp.pop_front()
    }

    pub fn read_outp_ascii(&mut self) -> String {
        let mut s = String::new();
        while let Some(i) = self.outp.pop_front() {
            s.push(char::from_u32(i as u32).unwrap());
        }
        s
    }

    pub fn read_outp_all(&mut self) -> Vec<IntSize> {
        self.outp.drain(..).collect()
    }

    pub fn mem_at(&self, a: IntSize) -> IntSize {
        self.mem.load(a)
    }

    pub fn run(&mut self) {
        self.state = RunState::Running;
        while self.state == RunState::Running {
            if let Err(e) = self.op() {
                println!("Intcode Error: {}", e);
                self.state = RunState::Error;
            }
        }
    }

    fn addr(&self, n: usize, opcode: IntSize, a_in: IntSize) -> Result<IntSize, String> {
        let mode = AddrMode::from(n, opcode);
        match mode {
            AddrMode::Pos => Ok(self.mem.load(a_in)),
            AddrMode::Imm => Ok(a_in),
            AddrMode::Rel => Ok(self.rel_base + self.mem.load(a_in)),
            AddrMode::Invalid(i) => Err(format!("Invalid parameter mode: {}", i)),
        }
    }

    fn op(&mut self) -> Result<(), String> {
        let opcode = self.mem.load(self.pc);
        let a: Vec<_> = [0, 1, 2]
            .iter()
            .map(|n| self.addr(*n, opcode, self.pc + 1 + *n as IntSize))
            .collect::<Result<Vec<_>, _>>()?;

        match opcode % 100 {
            1 => {
                // add
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                self.mem.store(a[2], p1 + p2);
                self.pc += 4;
            }
            2 => {
                // mul
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                self.mem.store(a[2], p1 * p2);
                self.pc += 4;
            }
            3 => {
                if self.inp.is_empty() {
                    self.state = RunState::Blocked;
                } else {
                    self.mem.store(a[0], self.inp.pop_front().unwrap());
                    self.pc += 2;
                }
            }
            4 => {
                let p1 = self.mem.load(a[0]);
                self.outp.push_back(p1);
                self.pc += 2;
            }
            5 => {
                // jnz
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                if p1 != 0 {
                    self.pc = p2;
                } else {
                    self.pc += 3;
                }
            }
            6 => {
                // jz
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                if p1 == 0 {
                    self.pc = p2;
                } else {
                    self.pc += 3;
                }
            }
            7 => {
                // lt
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                let res = if p1 < p2 { 1 } else { 0 };
                self.mem.store(a[2], res);
                self.pc += 4;
            }
            8 => {
                // eq
                let p1 = self.mem.load(a[0]);
                let p2 = self.mem.load(a[1]);
                let res = if p1 == p2 { 1 } else { 0 };
                self.mem.store(a[2], res);
                self.pc += 4;
            }
            9 => {
                // modify rel_base
                let p1 = self.mem.load(a[0]);
                self.rel_base += p1;
                self.pc += 2;
            }
            99 => {
                self.state = RunState::Halted;
            }
            _ => {
                return Err(format!("Invalid opcode: {}", opcode));
            }
        }

        Ok(())
    }
}
