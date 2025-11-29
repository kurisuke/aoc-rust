use std::collections::{BTreeSet, HashMap};

use common::day::Day;
use itertools::Itertools;

pub struct Day24 {}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let (gates, mut values) = parse_input(input);
        output_number(&gates, &mut values).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let (gates, _) = parse_input(input);
        let swapped = swapped_gates(&gates);
        swapped.iter().join(",")
    }
}

type Gates<'a> = HashMap<&'a str, Gate<'a>>;
type Values<'a> = HashMap<&'a str, bool>;

#[derive(PartialEq, Eq, Hash)]
struct Gate<'a> {
    pub input: (&'a str, &'a str),
    pub op: Op,
}

#[derive(PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

fn solve<'a>(gate_id: &'a str, gates: &'a Gates, values: &mut Values<'a>) -> bool {
    if let Some(v) = values.get(gate_id) {
        return *v;
    }

    let gate = gates.get(gate_id).unwrap();
    let i1 = solve(gate.input.0, gates, values);
    let i2 = solve(gate.input.1, gates, values);

    let out = match gate.op {
        Op::And => i1 & i2,
        Op::Or => i1 | i2,
        Op::Xor => i1 ^ i2,
    };
    values.insert(gate_id, out);
    out
}

fn parse_input(input: &str) -> (Gates<'_>, Values<'_>) {
    let mut secs = input.split("\n\n");

    let values = secs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            (parts[0], parts[1].starts_with("1"))
        })
        .collect();

    let gates = secs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let output = parts[4];
            let input = (parts[0], parts[2]);
            let op = match parts[1] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => unreachable!(),
            };
            (output, Gate { input, op })
        })
        .collect();

    (gates, values)
}

fn output_number<'a>(gates: &'a Gates, values: &mut Values<'a>) -> usize {
    let mut output = 0;

    for z_gate in gates.keys().filter(|key| key.starts_with('z')) {
        let index: usize = z_gate[1..].parse().unwrap();
        let value = solve(z_gate, gates, values);

        if value {
            output += 1 << index;
        }
    }

    output
}

fn swapped_gates<'a>(gates: &Gates<'a>) -> BTreeSet<&'a str> {
    let mut swapped = BTreeSet::new();

    let z_max: usize = gates
        .keys()
        .filter(|&k| k.starts_with('z'))
        .map(|&k| k[1..].parse().unwrap())
        .max()
        .unwrap();
    let z_max = format!("z{:02}", z_max);

    // z values must come from XOR gates (except z_max, which is the carry of the final full adder)
    for (&output, gate) in gates
        .iter()
        .filter(|(&output, _)| output.starts_with('z') && output != z_max)
    {
        if gate.op != Op::Xor {
            swapped.insert(output);
        }
    }

    for (&output, gate) in gates {
        match gate.op {
            Op::Xor => {
                // - XOR gates must be either connected to input (x,y) or output
                // - XOR gates cannot feed into OR gate
                if !(output.starts_with('z')
                    || gate.input.0.starts_with(['x', 'y'])
                    || gate.input.1.starts_with(['x', 'y']))
                    || gates.values().any(|gate2| {
                        (gate2.input.0 == output || gate2.input.1 == output) && gate2.op == Op::Or
                    })
                {
                    swapped.insert(output);
                }
            }
            Op::And => {
                // AND gates (except for first bit, which is only a half-adder) must feed into OR gate
                if (gate.input.0 != "x00" && gate.input.1 != "x00")
                    && gates.values().any(|gate2| {
                        (gate2.input.0 == output || gate2.input.1 == output) && gate2.op != Op::Or
                    })
                {
                    swapped.insert(output);
                }
            }
            Op::Or => {}
        }
    }

    swapped
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    const INPUT2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn star1() {
        let d = Day24 {};
        assert_eq!(d.star1(INPUT1), "4");
        assert_eq!(d.star1(INPUT2), "2024");
    }
}
