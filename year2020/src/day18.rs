use common::day::Day;

pub struct Day18 {}

#[derive(PartialEq)]
enum Token {
    Num(i64),
    Op(Operator),
    ParenLeft,
    ParenRight,
}

#[derive(PartialEq)]
enum Operator {
    Plus,
    Mul,
}

fn precedence_part1(op: &Operator) -> usize {
    match op {
        Operator::Plus => 1,
        Operator::Mul => 1,
    }
}

fn precedence_part2(op: &Operator) -> usize {
    match op {
        Operator::Plus => 2,
        Operator::Mul => 1,
    }
}

fn parse_token(input: &str) -> Option<Token> {
    if input.parse::<i64>().is_ok() {
        Some(Token::Num(input.parse::<i64>().unwrap()))
    } else if input.len() > 1 || input.is_empty() {
        None
    } else {
        match input.chars().next().unwrap() {
            '+' => Some(Token::Op(Operator::Plus)),
            '*' => Some(Token::Op(Operator::Mul)),
            '(' => Some(Token::ParenLeft),
            ')' => Some(Token::ParenRight),
            _ => None,
        }
    }
}

fn convert_rpn<F>(input: &[&str], precedence_fn: &F) -> Vec<Token>
where
    F: Fn(&Operator) -> usize,
{
    let mut op_stack = vec![];
    let mut output = vec![];

    for token in input {
        let token = parse_token(token).unwrap();
        match &token {
            Token::Num(_) => {
                output.push(token);
            }
            Token::Op(op) => {
                while !op_stack.is_empty() {
                    let top = op_stack.last().unwrap();
                    match top {
                        Token::ParenLeft => {
                            break;
                        }
                        Token::Op(top_op) => {
                            if precedence_fn(top_op) < precedence_fn(op) {
                                break;
                            }
                        }
                        _ => {}
                    }
                    output.push(op_stack.pop().unwrap());
                }
                op_stack.push(token);
            }
            Token::ParenLeft => {
                op_stack.push(token);
            }
            Token::ParenRight => {
                while op_stack.last().unwrap() != &Token::ParenLeft {
                    output.push(op_stack.pop().unwrap());
                }
                op_stack.pop().unwrap();
            }
        }
    }

    while !op_stack.is_empty() {
        output.push(op_stack.pop().unwrap());
    }

    output
}

fn eval_rpn(expr: &[Token]) -> i64 {
    let mut num_stack = vec![];
    for token in expr {
        match &token {
            Token::Num(n) => {
                num_stack.push(*n);
            }
            Token::Op(op) => {
                let p1 = num_stack.pop().unwrap();
                let p2 = num_stack.pop().unwrap();
                let res = match op {
                    Operator::Plus => p1 + p2,
                    Operator::Mul => p1 * p2,
                };
                num_stack.push(res);
            }
            Token::ParenLeft => {
                panic!("( in eval!");
            }
            Token::ParenRight => {
                panic!("( in eval!");
            }
        }
    }
    num_stack.pop().unwrap()
}

fn parse_input<F>(input: &str, precedence_fn: &F) -> Vec<Vec<Token>>
where
    F: Fn(&Operator) -> usize,
{
    input
        .lines()
        .map(|l| {
            let l = l.replace('(', " ( ");
            let l = l.replace(')', " ) ");
            let tokens: Vec<_> = l.split_whitespace().collect();
            convert_rpn(&tokens, precedence_fn)
        })
        .collect()
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let exprs = parse_input(input, &precedence_part1);
        let sum: i64 = exprs.iter().map(|e| eval_rpn(e)).sum();
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let exprs = parse_input(input, &precedence_part2);
        let sum: i64 = exprs.iter().map(|e| eval_rpn(e)).sum();
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_part1() {
        let d = Day18 {};
        assert_eq!(d.star1("1 + 2 * 3 + 4 * 5 + 6"), "71");
        assert_eq!(d.star1("1 + (2 * 3) + (4 * (5 + 6))"), "51");
        assert_eq!(d.star1("2 * 3 + (4 * 5)"), "26");
        assert_eq!(d.star1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), "437");
        assert_eq!(
            d.star1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            "12240"
        );
        assert_eq!(
            d.star1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            "13632"
        );
    }

    #[test]
    fn ex_part2() {
        let d = Day18 {};
        assert_eq!(d.star2("1 + 2 * 3 + 4 * 5 + 6"), "231");
        assert_eq!(d.star2("1 + (2 * 3) + (4 * (5 + 6))"), "51");
        assert_eq!(d.star2("2 * 3 + (4 * 5)"), "46");
        assert_eq!(d.star2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), "1445");
        assert_eq!(
            d.star2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            "669060"
        );
        assert_eq!(
            d.star2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            "23340"
        );
    }
}
