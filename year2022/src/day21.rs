use std::collections::HashMap;

use common::day::Day;

pub struct Day21 {}

pub enum Node<'a> {
    Num(u64),
    Op(OpType, &'a str, &'a str),
}

pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}

type Nodes<'a> = HashMap<&'a str, Node<'a>>;

fn parse_input(input: &str) -> Nodes<'_> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            let tokens: Vec<_> = parts[1].split_whitespace().collect();
            let node = match tokens.len() {
                1 => Node::Num(tokens[0].parse().unwrap()),
                3 => {
                    let op_type = match tokens[1] {
                        "+" => OpType::Add,
                        "-" => OpType::Sub,
                        "*" => OpType::Mul,
                        "/" => OpType::Div,
                        _ => unreachable!(),
                    };
                    Node::Op(op_type, tokens[0], tokens[2])
                }
                _ => unreachable!(),
            };
            (parts[0], node)
        })
        .collect()
}

fn eval_node(nodes: &Nodes, id: &str) -> Option<u64> {
    if let Some(node) = nodes.get(id) {
        match node {
            Node::Num(x) => Some(*x),
            Node::Op(op_type, id1, id2) => {
                let (n1, n2) = (eval_node(nodes, id1), eval_node(nodes, id2));
                if let (Some(n1), Some(n2)) = (n1, n2) {
                    match op_type {
                        OpType::Add => Some(n1 + n2),
                        OpType::Sub => Some(n1 - n2),
                        OpType::Mul => Some(n1 * n2),
                        OpType::Div => Some(n1 / n2),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn path<'a>(nodes: &'a Nodes, src: &'a str, dest: &'a str) -> Vec<&'a str> {
    if src == dest {
        vec![src]
    } else {
        match nodes.get(src).unwrap() {
            Node::Num(_) => vec![],
            Node::Op(_, id1, id2) => {
                for id in [id1, id2] {
                    let mut v = path(nodes, id, dest);
                    if !v.is_empty() {
                        v.push(src);
                        return v;
                    }
                }
                vec![]
            }
        }
    }
}

fn solve(nodes: &Nodes, stack: &mut Vec<&str>, target_num: u64) -> u64 {
    if stack.len() == 1 {
        target_num
    } else {
        let node = nodes.get(&stack.pop().unwrap()).unwrap();
        match node {
            Node::Num(_) => unreachable!(),
            Node::Op(op_type, n1, n2) => {
                let (is_first, other_operand) = if n1 == stack.last().unwrap() {
                    (true, eval_node(nodes, n2).unwrap())
                } else {
                    (false, eval_node(nodes, n1).unwrap())
                };

                let target_num_new = match op_type {
                    OpType::Add => target_num - other_operand,
                    OpType::Sub => {
                        if is_first {
                            target_num + other_operand
                        } else {
                            other_operand - target_num
                        }
                    }
                    OpType::Mul => target_num / other_operand,
                    OpType::Div => {
                        if is_first {
                            target_num * other_operand
                        } else {
                            other_operand / target_num
                        }
                    }
                };
                solve(nodes, stack, target_num_new)
            }
        }
    }
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let nodes = parse_input(input);
        format!("{}", eval_node(&nodes, "root").unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let nodes = parse_input(input);
        let mut path_humn = path(&nodes, "root", "humn");
        path_humn.pop();
        if let Some(Node::Op(_, id1, id2)) = nodes.get("root") {
            let target_num = if path_humn.last().unwrap() == id1 {
                eval_node(&nodes, id2).unwrap()
            } else {
                eval_node(&nodes, id1).unwrap()
            };
            format!("{}", solve(&nodes, &mut path_humn, target_num))
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

        let d = Day21 {};
        assert_eq!(d.star1(input), "152");
        assert_eq!(d.star2(input), "301");
    }
}
