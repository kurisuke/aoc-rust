use common::day::Day;

pub struct Day07 {}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let equations = parse_input(input);
        let calibration_value = equations
            .into_iter()
            .filter_map(|eq| {
                if evaluate(eq.test_value, eq.operands) {
                    Some(eq.test_value)
                } else {
                    None
                }
            })
            .sum::<usize>();
        format!("{}", calibration_value)
    }

    fn star2(&self, input: &str) -> String {
        let equations = parse_input(input);
        let calibration_value = equations
            .into_iter()
            .filter_map(|eq| {
                if evaluate2(eq.test_value, eq.operands) {
                    Some(eq.test_value)
                } else {
                    None
                }
            })
            .sum::<usize>();
        format!("{}", calibration_value)
    }
}

struct Equation {
    test_value: usize,
    operands: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(": ");
            let test_value = spl.next().unwrap().parse().unwrap();
            let mut operands: Vec<_> = spl
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            operands.reverse();
            Equation {
                test_value,
                operands,
            }
        })
        .collect()
}

fn evaluate(test_value: usize, mut operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        test_value == operands[0]
    } else {
        let op1 = operands.pop().unwrap();
        let op2 = operands.pop().unwrap();

        let mut operands_add = operands.clone();
        operands_add.push(op1 + op2);
        if evaluate(test_value, operands_add) {
            return true;
        }

        let mut operands_mul = operands;
        operands_mul.push(op1 * op2);
        evaluate(test_value, operands_mul)
    }
}

fn evaluate2(test_value: usize, mut operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        test_value == operands[0]
    } else {
        let op1 = operands.pop().unwrap();
        let op2 = operands.pop().unwrap();

        let mut operands_add = operands.clone();
        operands_add.push(op1 + op2);
        if evaluate2(test_value, operands_add) {
            return true;
        }

        let mut operands_mul = operands.clone();
        operands_mul.push(op1 * op2);
        if evaluate2(test_value, operands_mul) {
            return true;
        }

        let mut operands_concat = operands;
        operands_concat.push(concat(op1, op2));
        evaluate2(test_value, operands_concat)
    }
}

fn concat(mut x: usize, y: usize) -> usize {
    let mut y_tmp = y;
    loop {
        x *= 10;
        y_tmp /= 10;
        if y_tmp == 0 {
            break;
        }
    }
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn star1() {
        let d = Day07 {};
        assert_eq!(d.star1(INPUT), "3749");
    }

    #[test]
    fn star2() {
        let d = Day07 {};
        assert_eq!(d.star2(INPUT), "11387");
    }
}
