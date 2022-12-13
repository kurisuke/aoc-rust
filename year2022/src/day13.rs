use common::day::Day;

use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Term {
    Int(usize),
    List(Vec<Term>),
}

impl Term {
    fn parse(line: &str) -> Term {
        let line = line.replace('[', " [ ");
        let line = line.replace(']', " ] ");
        let line = line.replace(',', " , ");

        let mut op_stack = vec![];

        for token in line.split_whitespace() {
            match token {
                "[" => {
                    op_stack.push(Term::List(vec![]));
                }
                "," | "]" => {
                    let insert_src = op_stack.pop().unwrap();
                    if let Some(insert_dest) = op_stack.pop() {
                        if let Term::List(mut v) = insert_dest {
                            v.push(insert_src);
                            op_stack.push(Term::List(v));
                        } else {
                            unreachable!();
                        }
                    } else {
                        op_stack.push(insert_src);
                    }
                }
                _ => {
                    op_stack.push(Term::Int(token.parse().unwrap()));
                }
            }
        }

        op_stack[0].clone()
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Term::Int(a), Term::Int(b)) => a.cmp(b),
            (Term::List(la), Term::List(lb)) => {
                for (a, b) in la.iter().zip(lb.iter()) {
                    let o = a.cmp(b);
                    if o != Ordering::Equal {
                        return o;
                    }
                }
                la.len().cmp(&lb.len())
            }
            (Term::Int(_), Term::List(_)) => {
                let la = Term::List(vec![self.clone()]);
                la.cmp(other)
            }
            (Term::List(_), Term::Int(_)) => {
                let lb = Term::List(vec![other.clone()]);
                self.cmp(&lb)
            }
        }
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day13 {}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let sum: usize = input
            .split("\n\n")
            .enumerate()
            .map(|(idx, pair)| {
                let mut lines = pair.lines();
                let first = lines.next().unwrap();
                let second = lines.next().unwrap();
                if Term::parse(first) < Term::parse(second) {
                    idx + 1
                } else {
                    0
                }
            })
            .sum();
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let mut packets: Vec<_> = input
            .lines()
            .filter_map(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(Term::parse(l))
                }
            })
            .collect();

        let divider1 = Term::parse("[[2]]");
        let divider2 = Term::parse("[[6]]");

        packets.push(divider1.clone());
        packets.push(divider2.clone());

        packets.sort();

        let pos1 = packets.iter().position(|p| *p == divider1).unwrap() + 1;
        let pos2 = packets.iter().position(|p| *p == divider2).unwrap() + 1;

        format!("{}", pos1 * pos2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn term_cmp() {
        assert!(Term::parse("[1,1,3,1,1]") < Term::parse("[1,1,5,1,1]"));
        assert!(Term::parse("[[1],[2,3,4]]") < Term::parse("[[1],4]"));
        assert!(Term::parse("[9]") > Term::parse("[[8,7,6]]"));
        assert!(Term::parse("[[4,4],4,4]") < Term::parse("[[4,4],4,4,4]"));
        assert!(Term::parse("[7,7,7,7]") > Term::parse("[7,7,7]"));
        assert!(Term::parse("[]") < Term::parse("[3]"));
        assert!(Term::parse("[[[]]]") > Term::parse("[[]]"));
        assert!(
            Term::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]") > Term::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }

    #[test]
    fn ex1() {
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let d = Day13 {};
        assert_eq!(d.star1(input), "13");
        assert_eq!(d.star2(input), "140");
    }
}
