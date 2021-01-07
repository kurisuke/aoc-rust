use common::day::Day;
use std::collections::HashMap;

pub struct Day07 {}

enum Op {
    Nop,
    Not,
    And,
    Or,
    Lshift,
    Rshift,
}

enum Input<'a> {
    Value(u16),
    Gate(&'a str),
}

struct Gate<'a> {
    inputs: Vec<Input<'a>>,
    op: Op,
}

type Gates<'a> = HashMap<&'a str, Gate<'a>>;
type Results = HashMap<String, u16>;

fn eval(gates: &Gates, results: &mut Results, id: &str) -> u16 {
    if let Some(result) = results.get(id) {
        return *result;
    }

    let gate = gates.get(id).unwrap();
    let in_val: Vec<_> = gate
        .inputs
        .iter()
        .map(|i| match i {
            Input::Value(x) => *x,
            Input::Gate(id) => eval(&gates, results, id),
        })
        .collect();

    let result = match gate.op {
        Op::Nop => in_val[0],
        Op::Not => !in_val[0],
        Op::And => in_val[0] & in_val[1],
        Op::Or => in_val[0] | in_val[1],
        Op::Lshift => in_val[0] << in_val[1],
        Op::Rshift => in_val[0] >> in_val[1],
    };
    results.insert(String::from(id), result);
    result
}

fn parse_input_val(s: &str) -> Input {
    if let Ok(v) = s.parse::<u16>() {
        Input::Value(v)
    } else {
        Input::Gate(s)
    }
}

fn parse_input(input: &str) -> Gates {
    let mut gates = HashMap::new();
    for line in input.lines() {
        let mut it = line.split(" -> ");
        let input_str = it.next().unwrap();
        let id = it.next().unwrap();

        let inputs: Vec<_> = input_str.split(' ').collect();
        match inputs.len() {
            1 => {
                // NOP
                let input_vals = vec![parse_input_val(inputs[0])];
                gates.insert(
                    id,
                    Gate {
                        op: Op::Nop,
                        inputs: input_vals,
                    },
                );
            }
            2 => {
                // NOT
                let input_vals = vec![parse_input_val(inputs[1])];
                let op = match inputs[0] {
                    "NOT" => Op::Not,
                    _ => {
                        panic!("cannot parse input string: {}", input_str);
                    }
                };
                gates.insert(
                    id,
                    Gate {
                        op,
                        inputs: input_vals,
                    },
                );
            }
            3 => {
                let input_vals = vec![parse_input_val(inputs[0]), parse_input_val(inputs[2])];
                let op = match inputs[1] {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "LSHIFT" => Op::Lshift,
                    "RSHIFT" => Op::Rshift,
                    _ => {
                        panic!("cannot parse input string: {}", input_str);
                    }
                };
                gates.insert(
                    id,
                    Gate {
                        op,
                        inputs: input_vals,
                    },
                );
            }
            _ => {
                panic!("cannot parse input string: {}", input_str);
            }
        }
    }
    gates
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let gates = parse_input(input);
        let mut results = HashMap::new();
        format!("{}", eval(&gates, &mut results, "a"))
    }

    fn star2(&self, input: &str) -> String {
        let val_part1 = self.star1(input).parse::<u16>().unwrap();
        let mut gates = parse_input(input);
        gates.entry("b").and_modify(|e| {
            *e = Gate {
                inputs: vec![Input::Value(val_part1)],
                op: Op::Nop,
            }
        });
        let mut results = HashMap::new();
        format!("{}", eval(&gates, &mut results, "a"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let gates = parse_input(input);
        let mut results = HashMap::new();
        assert_eq!(eval(&gates, &mut results, "d"), 72);
        assert_eq!(eval(&gates, &mut results, "e"), 507);
        assert_eq!(eval(&gates, &mut results, "f"), 492);
        assert_eq!(eval(&gates, &mut results, "g"), 114);
        assert_eq!(eval(&gates, &mut results, "h"), 65412);
        assert_eq!(eval(&gates, &mut results, "i"), 65079);
        assert_eq!(eval(&gates, &mut results, "x"), 123);
        assert_eq!(eval(&gates, &mut results, "y"), 456);
    }
}
