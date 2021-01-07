use common::day::Day;
use std::collections::HashMap;

pub struct Day08 {}

enum Op {
    Inc,
    Dec,
}

enum Rel {
    LT,
    LE,
    Eq,
    NEq,
    GT,
    GE,
}

struct Instr {
    reg: String,
    op: Op,
    arg: isize,
    cond_reg: String,
    cond_rel: Rel,
    cond_arg: isize,
}

fn parse_op(s: &str) -> Op {
    match s {
        "inc" => Op::Inc,
        "dec" => Op::Dec,
        _ => {
            panic!("Unknown operation: {}", s);
        }
    }
}

fn parse_rel(s: &str) -> Rel {
    match s {
        "<" => Rel::LT,
        "<=" => Rel::LE,
        "==" => Rel::Eq,
        "!=" => Rel::NEq,
        ">" => Rel::GT,
        ">=" => Rel::GE,
        _ => {
            panic!("Unknown relation: {}", s);
        }
    }
}

fn parse_instr(s: &str) -> Instr {
    let w: Vec<_> = s.split_whitespace().collect();
    assert_eq!(w.len(), 7);
    let reg = w[0].to_string();
    let op = parse_op(w[1]);
    let arg = w[2].parse::<isize>().unwrap();
    let cond_reg = w[4].to_string();
    let cond_rel = parse_rel(w[5]);
    let cond_arg = w[6].parse::<isize>().unwrap();
    Instr {
        reg,
        op,
        arg,
        cond_reg,
        cond_rel,
        cond_arg,
    }
}

fn eval(regs: &mut HashMap<String, isize>, instr: &Instr) {
    // check condition
    let reg_val = *regs.get(&instr.cond_reg).unwrap_or(&0);
    let execute = match instr.cond_rel {
        Rel::LT => reg_val < instr.cond_arg,
        Rel::LE => reg_val <= instr.cond_arg,
        Rel::Eq => reg_val == instr.cond_arg,
        Rel::NEq => reg_val != instr.cond_arg,
        Rel::GT => reg_val > instr.cond_arg,
        Rel::GE => reg_val >= instr.cond_arg,
    };
    if execute {
        let e = regs.entry(instr.reg.clone()).or_insert(0);
        match instr.op {
            Op::Inc => {
                *e += instr.arg;
            }
            Op::Dec => {
                *e -= instr.arg;
            }
        }
    }
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let instrs: Vec<_> = input.lines().map(|line| parse_instr(line)).collect();
        let mut regs = HashMap::new();
        for instr in instrs.iter() {
            eval(&mut regs, instr);
        }
        let largest = regs.values().max().unwrap();
        format!("{}", largest)
    }

    fn star2(&self, input: &str) -> String {
        let instrs: Vec<_> = input.lines().map(|line| parse_instr(line)).collect();
        let mut regs = HashMap::new();
        let mut largest_ever = std::isize::MIN;
        for instr in instrs.iter() {
            eval(&mut regs, instr);
            if !regs.is_empty() {
                let largest = regs.values().max().unwrap();
                largest_ever = largest_ever.max(*largest);
            }
        }
        format!("{}", largest_ever)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day08 {};
        let input = r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#;
        assert_eq!(d.star1(input), "1");
        assert_eq!(d.star2(input), "10");
    }
}
