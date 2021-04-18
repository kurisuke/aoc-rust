use common::day::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day16 {}

type Instruction = [usize; 4];
type Registers = [usize; 4];

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

const OPS: [Op; 16] = [
    Op::Addr,
    Op::Addi,
    Op::Mulr,
    Op::Muli,
    Op::Banr,
    Op::Bani,
    Op::Borr,
    Op::Bori,
    Op::Setr,
    Op::Seti,
    Op::Gtir,
    Op::Gtri,
    Op::Gtrr,
    Op::Eqir,
    Op::Eqri,
    Op::Eqrr,
];

struct Sample {
    instr: Instruction,
    before: Registers,
    after: Registers,
}

fn exec_op(instr: &Instruction, regs_in: &Registers, try_op: Op) -> Option<Registers> {
    let mut regs_out = *regs_in;
    match try_op {
        Op::Addr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] + regs_in[instr[2]];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Addi => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] + instr[2];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Mulr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] * regs_in[instr[2]];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Muli => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] * instr[2];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Banr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] & regs_in[instr[2]];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Bani => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] & instr[2];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Borr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] | regs_in[instr[2]];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Bori => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]] | instr[2];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Setr => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = regs_in[instr[1]];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Seti => {
            if instr[3] < 4 {
                regs_out[instr[3]] = instr[1];
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtir => {
            if instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if instr[1] > regs_in[instr[2]] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtri => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if regs_in[instr[1]] > instr[2] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Gtrr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if regs_in[instr[1]] > regs_in[instr[2]] {
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
            if instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if instr[1] == regs_in[instr[2]] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Eqri => {
            if instr[1] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if regs_in[instr[1]] == instr[2] { 1 } else { 0 };
                Some(regs_out)
            } else {
                None
            }
        }
        Op::Eqrr => {
            if instr[1] < 4 && instr[2] < 4 && instr[3] < 4 {
                regs_out[instr[3]] = if regs_in[instr[1]] == regs_in[instr[2]] {
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

fn possible_ops(sample: &Sample) -> Vec<Op> {
    OPS.iter()
        .filter(|op| {
            let calc_after = exec_op(&sample.instr, &sample.before, **op);
            calc_after.is_some() && calc_after.unwrap() == sample.after
        })
        .cloned()
        .collect()
}

fn possible_ops_mult(samples: &[&Sample]) -> Vec<Op> {
    OPS.iter()
        .filter(|op| {
            samples.iter().all(|sample| {
                let calc_after = exec_op(&sample.instr, &sample.before, **op);
                calc_after.is_some() && calc_after.unwrap() == sample.after
            })
        })
        .cloned()
        .collect()
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let mut it_parts = input.split("\n\n\n\n");
    let mut samples = vec![];
    let mut program = vec![];

    let re_before = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let re_after = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    if let Some(sample_part) = it_parts.next() {
        for sample_str in sample_part.split("\n\n") {
            let sample_lines: Vec<_> = sample_str.lines().collect();

            let caps = re_before.captures(sample_lines[0]).unwrap();
            let before = [
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            ];

            let instr_str: Vec<_> = sample_lines[1].split_whitespace().collect();
            let instr = [
                instr_str[0].parse::<usize>().unwrap(),
                instr_str[1].parse::<usize>().unwrap(),
                instr_str[2].parse::<usize>().unwrap(),
                instr_str[3].parse::<usize>().unwrap(),
            ];

            let caps = re_after.captures(sample_lines[2]).unwrap();
            let after = [
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            ];

            samples.push(Sample {
                instr,
                before,
                after,
            });
        }
    }

    if let Some(instr_part) = it_parts.next() {
        for instr_line in instr_part.lines() {
            let instr_str: Vec<_> = instr_line.split_whitespace().collect();
            program.push([
                instr_str[0].parse::<usize>().unwrap(),
                instr_str[1].parse::<usize>().unwrap(),
                instr_str[2].parse::<usize>().unwrap(),
                instr_str[3].parse::<usize>().unwrap(),
            ]);
        }
    }

    (samples, program)
}

fn determine_ops(samples: &[Sample]) -> HashMap<usize, Op> {
    // sort samples by opcode
    let mut samples_by_opcode = HashMap::new();

    for sample in samples {
        let opcode = sample.instr[0];
        let entry = samples_by_opcode.entry(opcode).or_insert_with(Vec::new);
        entry.push(sample);
    }

    let mut possible_by_opcode = HashMap::new();

    for (opcode, opcode_samples) in samples_by_opcode.iter() {
        let possible = possible_ops_mult(opcode_samples);
        possible_by_opcode.insert(opcode, possible);
    }

    let mut determined = HashSet::new();
    while determined.len() < 16 {
        let next_determined = possible_by_opcode
            .values()
            .find(|p| p.len() == 1 && !determined.contains(&p[0]))
            .unwrap()[0];

        for p in possible_by_opcode.values_mut() {
            if p.len() > 1 {
                p.retain(|x| *x != next_determined);
            }
        }
        determined.insert(next_determined);
    }

    possible_by_opcode
        .into_iter()
        .map(|(k, v)| (*k, v[0]))
        .collect()
}

fn run_program(program: &[Instruction], ops: &HashMap<usize, Op>) -> Registers {
    let mut reg = [0, 0, 0, 0];
    for instr in program.iter() {
        let opcode = instr[0];
        let op = ops.get(&opcode).unwrap();
        reg = exec_op(instr, &reg, *op).unwrap();
    }
    reg
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let (samples, _program) = parse_input(input);
        format!(
            "{}",
            samples
                .iter()
                .filter(|sample| possible_ops(sample).len() >= 3)
                .count()
        )
    }

    fn star2(&self, input: &str) -> String {
        let (samples, program) = parse_input(input);
        let ops = determine_ops(&samples);
        let final_regs = run_program(&program, &ops);
        format!("{}", final_regs[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#;
        let (samples, _program) = parse_input(input);
        let ops = possible_ops(&samples[0]);
        assert_eq!(ops[0], Op::Addi);
        assert_eq!(ops[1], Op::Mulr);
        assert_eq!(ops[2], Op::Seti);
    }
}
