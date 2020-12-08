use crate::day::Day;
use scan_fmt::{parse::ScanError, scan_fmt};

pub struct Day08 {}

#[derive(Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

enum EndState {
    Loop(i64),
    JmpErr(i64),
    Succ(i64),
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let ins = parse_input(input);
        let endstate = run_program(&ins);
        match endstate {
            EndState::Loop(acc) => format!("{}", acc),
            _ => String::from("error"),
        }
    }

    fn star2(&self, input: &str) -> String {
        let ins = parse_input(input);

        for idx in 0..ins.len() {
            let ins_mod = modify_program(&ins, idx);
            let endstate = run_program(&ins_mod);
            match endstate {
                EndState::Succ(acc) => {
                    return format!("{}", acc);
                }
                _ => {}
            }
        }
        String::from("error")
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map::<Result<Instruction, ScanError>, _>(|line| {
            let (instr, sign, amount) = scan_fmt!(line, "{} {[+-]}{d}", String, char, i64)?;
            let amount = match sign {
                '+' => amount,
                '-' => -amount,
                _ => {
                    return Err(ScanError(format!("wrong sign: {}", sign)));
                }
            };
            let instr = match instr.as_ref() {
                "acc" => Instruction::Acc(amount),
                "jmp" => Instruction::Jmp(amount),
                "nop" => Instruction::Nop(amount),
                _ => {
                    return Err(ScanError(format!("wrong instruction: {}", instr)));
                }
            };
            Ok(instr)
        })
        .filter_map(Result::ok)
        .collect()
}

fn run_program(ins: &Vec<Instruction>) -> EndState {
    let mut already_run = vec![false; ins.len()];
    let mut pc = 0;
    let mut acc = 0;

    loop {
        if already_run[pc] {
            return EndState::Loop(acc);
        }
        already_run[pc] = true;
        match ins[pc] {
            Instruction::Acc(v) => {
                acc += v;
                pc += 1;
            }
            Instruction::Jmp(v) => {
                let pc_new = pc as i64 + v;
                if pc_new < 0 || pc_new > ins.len() as i64 {
                    return EndState::JmpErr(acc);
                } else {
                    pc = pc_new as usize;
                }
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
        if pc == ins.len() {
            return EndState::Succ(acc);
        }
    }
}

fn modify_program(ins: &Vec<Instruction>, idx: usize) -> Vec<Instruction> {
    let mut ins = ins.to_vec();
    match ins[idx] {
        Instruction::Nop(v) => {
            ins[idx] = Instruction::Jmp(v);
        }
        Instruction::Jmp(v) => {
            ins[idx] = Instruction::Nop(v);
        }
        Instruction::Acc(_) => {}
    }
    ins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let d = Day08 {};
        assert_eq!(d.star1(input), "5");
        assert_eq!(d.star2(input), "8");
    }
}
