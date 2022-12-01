#[derive(Copy, Clone)]
pub enum RegId {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone)]
pub enum Val {
    Imm(i64),
    Reg(RegId),
}

#[derive(Copy, Clone)]
pub enum Op {
    Nop,
    Cpy(Val, RegId),
    Inc(RegId),
    Dec(RegId),
    Jnz(Val, Val),
    Tgl(Val),
    Out(Val),
}

pub struct Computer {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    pc: i64,
    program: Vec<Op>,
}

impl Computer {
    pub fn new(program_str: &str) -> Computer {
        let program = parse_input(program_str);
        Computer {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            program,
        }
    }

    pub fn exec(&mut self, max_out: Option<usize>, max_op: Option<usize>) -> String {
        let mut op_count = 0;
        let mut out_buf = String::new();
        while self.pc >= 0
            && self.pc < self.program.len() as i64
            && (max_op.is_none() || op_count < max_op.unwrap())
            && (max_out.is_none() || out_buf.len() < max_out.unwrap())
        {
            op_count += 1;
            match self.program[self.pc as usize] {
                Op::Nop => {
                    self.pc += 1;
                }
                Op::Cpy(x, y) => {
                    self.set_reg(y, self.eval(&x));
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
                Op::Jnz(x, y) => {
                    if self.eval(&x) != 0 {
                        self.pc += self.eval(&y);
                    } else {
                        self.pc += 1;
                    }
                }
                Op::Tgl(x) => {
                    self.toggle(self.pc + self.eval(&x));
                    self.pc += 1;
                }
                Op::Out(x) => {
                    out_buf.push_str(&self.eval(&x).to_string());
                    self.pc += 1;
                }
            }
        }
        out_buf
    }

    pub fn get_reg(&self, id: RegId) -> i64 {
        match id {
            RegId::A => self.a,
            RegId::B => self.b,
            RegId::C => self.c,
            RegId::D => self.d,
        }
    }

    pub fn set_reg(&mut self, id: RegId, v: i64) {
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

    fn eval(&self, val: &Val) -> i64 {
        match val {
            Val::Imm(x) => *x,
            Val::Reg(r) => self.get_reg(*r),
        }
    }

    fn toggle(&mut self, pos: i64) {
        if pos >= 0 && pos < self.program.len() as i64 {
            let new_op = match self.program[pos as usize] {
                Op::Nop => Op::Nop,
                Op::Cpy(x, y) => Op::Jnz(x, Val::Reg(y)),
                Op::Inc(x) => Op::Dec(x),
                Op::Dec(x) => Op::Inc(x),
                Op::Jnz(x, y) => match y {
                    Val::Imm(_) => Op::Nop,
                    Val::Reg(r) => Op::Cpy(x, r),
                },
                Op::Tgl(x) => match x {
                    Val::Imm(_) => Op::Nop,
                    Val::Reg(r) => Op::Inc(r),
                },
                Op::Out(x) => match x {
                    Val::Imm(_) => Op::Nop,
                    Val::Reg(r) => Op::Inc(r),
                },
            };
            self.program[pos as usize] = new_op;
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

fn parse_val(s: &str) -> Val {
    match s.parse::<i64>() {
        Ok(x) => Val::Imm(x),
        Err(_) => Val::Reg(to_regid(s).unwrap()),
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .filter_map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            match words[0] {
                "cpy" => Some(Op::Cpy(parse_val(words[1]), to_regid(words[2]).unwrap())),
                "inc" => Some(Op::Inc(to_regid(words[1]).unwrap())),
                "dec" => Some(Op::Dec(to_regid(words[1]).unwrap())),
                "jnz" => Some(Op::Jnz(parse_val(words[1]), parse_val(words[2]))),
                "tgl" => Some(Op::Tgl(parse_val(words[1]))),
                "out" => Some(Op::Out(parse_val(words[1]))),
                _ => None,
            }
        })
        .collect()
}
