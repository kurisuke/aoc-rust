use common::day::Day;
use std::fmt;

pub struct Day18 {}

#[derive(Eq, PartialEq, Clone)]
enum Token {
    BracketL,
    BracketR,
    Comma,
    Num(usize),
}

#[derive(Eq, PartialEq, Clone)]
struct SfNum {
    tokens: Vec<Token>,
}

impl SfNum {
    fn new(input: &str) -> SfNum {
        let tokens = input
            .trim()
            .chars()
            .map(|c| match c {
                '0'..='9' => Token::Num(c.to_digit(10).unwrap() as usize),
                '[' => Token::BracketL,
                ']' => Token::BracketR,
                ',' => Token::Comma,
                _ => unreachable!(),
            })
            .collect();
        SfNum { tokens }
    }

    fn explode(&mut self) -> bool {
        for (i, w) in self.tokens.windows(5).enumerate() {
            // find pair
            if w[0] == Token::BracketL
                && matches!(w[1], Token::Num(_))
                && w[2] == Token::Comma
                && matches!(w[3], Token::Num(_))
                && w[4] == Token::BracketR
            {
                // check nesting level
                let front = &self.tokens[..i];
                let nesting_level = front.iter().filter(|t| t == &&Token::BracketL).count()
                    - front.iter().filter(|t| t == &&Token::BracketR).count();
                if nesting_level >= 4 {
                    let mut front: Vec<_> = self.tokens[..i].to_vec();
                    let mut back: Vec<_> = self.tokens[i + 5..].to_vec();
                    let lval = match w[1] {
                        Token::Num(n) => n,
                        _ => unreachable!(),
                    };
                    let rval = match w[3] {
                        Token::Num(n) => n,
                        _ => unreachable!(),
                    };

                    // add lval
                    for t in front.iter_mut().rev() {
                        if let Token::Num(n) = t {
                            *t = Token::Num(*n + lval);
                            break;
                        }
                    }

                    // add rval
                    for t in back.iter_mut() {
                        if let Token::Num(n) = t {
                            *t = Token::Num(*n + rval);
                            break;
                        }
                    }

                    front.push(Token::Num(0));
                    front.extend(back);
                    self.tokens = front;
                    return true;
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for (i, t) in self.tokens.iter().enumerate() {
            if let Token::Num(n) = t {
                if *n >= 10 {
                    let (lval, rval) = if *n % 2 != 0 {
                        // odd
                        (*n / 2, *n / 2 + 1)
                    } else {
                        // even
                        (*n / 2, *n / 2)
                    };
                    let middle = vec![
                        Token::BracketL,
                        Token::Num(lval),
                        Token::Comma,
                        Token::Num(rval),
                        Token::BracketR,
                    ];

                    let mut front: Vec<_> = self.tokens[..i].to_vec();
                    let back: Vec<_> = self.tokens[i + 1..].to_vec();
                    front.extend(middle);
                    front.extend(back);
                    self.tokens = front;
                    return true;
                }
            }
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &SfNum) {
        let mut tmp = vec![Token::BracketL];
        tmp.extend(self.tokens.iter().cloned());
        tmp.push(Token::Comma);
        tmp.extend(other.tokens.iter().cloned());
        tmp.push(Token::BracketR);
        self.tokens = tmp;
        self.reduce();
    }

    fn magnitude(&self) -> usize {
        let mut op_stack = vec![];
        let mut num_stack = vec![];

        for t in self.tokens.iter() {
            match t {
                Token::BracketL | Token::Comma => {
                    op_stack.push(t);
                }
                Token::Num(n) => {
                    num_stack.push(*n);
                }
                Token::BracketR => loop {
                    match op_stack.pop() {
                        None => unreachable!(),
                        Some(Token::Comma) => {
                            let rval = num_stack.pop().unwrap();
                            let lval = num_stack.pop().unwrap();
                            num_stack.push(3 * lval + 2 * rval);
                        }
                        Some(Token::BracketL) => {
                            break;
                        }
                        Some(_) => unreachable!(),
                    }
                },
            }
        }
        num_stack[0]
    }
}

fn sum(nums: &[SfNum]) -> SfNum {
    let mut sum = nums[0].clone();
    for other in nums.iter().skip(1) {
        sum.add(other);
    }
    sum
}

fn parse_input(input: &str) -> Vec<SfNum> {
    input.lines().map(SfNum::new).collect()
}

impl fmt::Display for SfNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tok in self.tokens.iter() {
            match tok {
                Token::BracketL => {
                    write!(f, "[")?;
                }
                Token::BracketR => {
                    write!(f, "]")?;
                }
                Token::Comma => {
                    write!(f, ",")?;
                }
                Token::Num(i) => {
                    write!(f, "{}", i)?;
                }
            }
        }
        Ok(())
    }
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let sf_nums = parse_input(input);
        let sf_sum = sum(&sf_nums);
        format!("{}", sf_sum.magnitude())
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let n = SfNum::new(s);
        let f = format!("{}", n);
        assert_eq!(s, f);
    }

    #[test]
    fn test_explode() {
        let s = "[[[[[9,8],1],2],3],4]";
        let mut n = SfNum::new(s);
        assert_eq!(n.explode(), true);
        assert_eq!(format!("{}", n), "[[[[0,9],2],3],4]");

        let s = "[7,[6,[5,[4,[3,2]]]]]";
        let mut n = SfNum::new(s);
        assert_eq!(n.explode(), true);
        assert_eq!(format!("{}", n), "[7,[6,[5,[7,0]]]]");

        let s = "[[6,[5,[4,[3,2]]]],1]";
        let mut n = SfNum::new(s);
        assert_eq!(n.explode(), true);
        assert_eq!(format!("{}", n), "[[6,[5,[7,0]]],3]");

        let s = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let mut n = SfNum::new(s);
        assert_eq!(n.explode(), true);
        assert_eq!(format!("{}", n), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let s = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let mut n = SfNum::new(s);
        assert_eq!(n.explode(), true);
        assert_eq!(format!("{}", n), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_split() {
        let mut n = SfNum {
            tokens: vec![Token::Num(10)],
        };
        assert_eq!(n.split(), true);
        assert_eq!(format!("{}", n), "[5,5]");

        let mut n = SfNum {
            tokens: vec![Token::Num(11)],
        };
        assert_eq!(n.split(), true);
        assert_eq!(format!("{}", n), "[5,6]");
    }

    #[test]
    fn test_add() {
        let mut n1 = SfNum::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = SfNum::new("[1,1]");

        n1.add(&n2);
        assert_eq!(format!("{}", n1), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_sum() {
        let input = r#"[1,1]
[2,2]
[3,3]
[4,4]"#;
        let sf_nums = parse_input(input);
        assert_eq!(
            format!("{}", sum(&sf_nums)),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );

        let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#;
        let sf_nums = parse_input(input);
        assert_eq!(
            format!("{}", sum(&sf_nums)),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );

        let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#;
        let sf_nums = parse_input(input);
        assert_eq!(
            format!("{}", sum(&sf_nums)),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );

        let input = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#;
        let sf_nums = parse_input(input);
        assert_eq!(
            format!("{}", sum(&sf_nums)),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(SfNum::new("[9,1]").magnitude(), 29);
        assert_eq!(SfNum::new("[[9,1],[1,9]]").magnitude(), 129);
        assert_eq!(SfNum::new("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            SfNum::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(SfNum::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(SfNum::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(
            SfNum::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            SfNum::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn ex1() {
        let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

        let d = Day18 {};
        assert_eq!(d.star1(input), "4140");
    }
}
