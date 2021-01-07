use common::day::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day25 {}

#[derive(Debug)]
struct InputInfo {
    begin_state: char,
    steps: usize,
    rules: TuringRules,
}

#[derive(Debug)]
struct TuringRule {
    write: bool,
    mov: isize,
    next_state: char,
}

type TuringRules = HashMap<char, (TuringRule, TuringRule)>;

struct TuringMachine {
    rules: TuringRules,
    pos: isize,
    state: char,
    pos_true: HashSet<isize>,
}

fn do_write(val: bool, pos: isize, pos_true: &mut HashSet<isize>) {
    if val {
        pos_true.insert(pos);
    } else {
        pos_true.remove(&pos);
    }
}

impl TuringMachine {
    fn run(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            let state_rules = self.rules.get(&self.state).unwrap();
            let rule = if self.pos_true.contains(&self.pos) {
                &state_rules.1
            } else {
                &state_rules.0
            };
            do_write(rule.write, self.pos, &mut self.pos_true);
            self.pos += rule.mov;
            self.state = rule.next_state;
        }
        self.pos_true.len()
    }
}

fn parse_input(input: &str) -> InputInfo {
    let re_begin_state = Regex::new(r"Begin in state (\w)\.").unwrap();
    let re_steps = Regex::new(r"Perform a diagnostic checksum after (\d+) steps\.").unwrap();
    let re_state_name = Regex::new(r"In state (\w):").unwrap();
    let re_write = Regex::new(r"Write the value (\d)\.").unwrap();
    let re_move = Regex::new(r"Move one slot to the (left|right)\.").unwrap();
    let re_next_state = Regex::new(r"Continue with state (\w)\.").unwrap();

    let secs: Vec<_> = input.split("\n\n").collect();

    let first_line = secs[0].lines().next().unwrap();
    let caps = re_begin_state.captures(first_line).unwrap();
    let begin_state = caps.get(1).unwrap().as_str().chars().next().unwrap();

    let second_line = secs[0].lines().nth(1).unwrap();
    let caps = re_steps.captures(second_line).unwrap();
    let steps = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let rules: TuringRules = secs[1..]
        .iter()
        .map(|sec| {
            let lines: Vec<_> = sec.lines().collect();
            let caps = re_state_name.captures(lines[0]).unwrap();
            let state_name = caps.get(1).unwrap().as_str().chars().next().unwrap();

            let caps = re_write.captures(lines[2]).unwrap();
            let write = caps.get(1).unwrap().as_str() == "1";

            let caps = re_move.captures(lines[3]).unwrap();
            let mov = if caps.get(1).unwrap().as_str() == "left" {
                -1
            } else {
                1
            };

            let caps = re_next_state.captures(lines[4]).unwrap();
            let next_state = caps.get(1).unwrap().as_str().chars().next().unwrap();

            let false_state = TuringRule {
                write,
                mov,
                next_state,
            };

            let caps = re_write.captures(lines[6]).unwrap();
            let write = caps.get(1).unwrap().as_str() == "1";

            let caps = re_move.captures(lines[7]).unwrap();
            let mov = if caps.get(1).unwrap().as_str() == "left" {
                -1
            } else {
                1
            };

            let caps = re_next_state.captures(lines[8]).unwrap();
            let next_state = caps.get(1).unwrap().as_str().chars().next().unwrap();

            let true_state = TuringRule {
                write,
                mov,
                next_state,
            };

            (state_name, (false_state, true_state))
        })
        .collect();
    InputInfo {
        begin_state,
        steps,
        rules,
    }
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let input_info = parse_input(input);
        let mut machine = TuringMachine {
            rules: input_info.rules,
            pos: 0,
            state: input_info.begin_state,
            pos_true: HashSet::new(),
        };
        format!("{:?}", machine.run(input_info.steps))
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
        let d = Day25 {};
        let input = r#"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A."#;
        assert_eq!(d.star1(input), "3");
    }
}
