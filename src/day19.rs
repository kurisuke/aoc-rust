use crate::day::Day;
use std::collections::HashMap;

pub struct Day19 {}

enum Rule {
    Literal(char),
    Sub(Vec<usize>, Option<Vec<usize>>),
}

type Ruleset = HashMap<usize, Rule>;

struct InputInfo<'a> {
    ruleset: Ruleset,
    msgs: Vec<&'a str>,
}

fn parse_rule(input: &str) -> Rule {
    if input.starts_with('"') {
        Rule::Literal(input.chars().nth(1).unwrap())
    } else {
        let mut spl = input.split(" | ");
        let first: Vec<_> = spl
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let second = match spl.next() {
            Some(s) => Some(
                s.split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            ),
            None => None,
        };
        Rule::Sub(first, second)
    }
}

fn parse_input(input: &str) -> InputInfo {
    let mut sections = input.split("\n\n");
    let rules_sec = sections.next().unwrap();

    let ruleset: HashMap<_, _> = rules_sec
        .lines()
        .map(|line| {
            let mut spl = line.split(": ");
            let id = spl.next().unwrap().parse::<usize>().unwrap();
            let rule = parse_rule(spl.next().unwrap());
            (id, rule)
        })
        .collect();

    let msgs_sec = sections.next().unwrap();
    let msgs: Vec<_> = msgs_sec.lines().collect();
    InputInfo { ruleset, msgs }
}

fn match_concat<'a>(ruleset: &Ruleset, rule_ids: &[usize], msg: &'a str) -> &'a str {
    let mut rest = msg;
    let mut match_ok = true;

    for rule_id in rule_ids {
        let new_rest = match_msg(ruleset, *rule_id, rest);
        if new_rest.len() < rest.len() {
            rest = new_rest;
        } else {
            match_ok = false;
            break;
        }
    }

    if match_ok {
        rest
    } else {
        msg
    }
}

fn match_msg<'a>(ruleset: &Ruleset, rule_id: usize, msg: &'a str) -> &'a str {
    if msg.is_empty() {
        return msg;
    }

    let rule = ruleset.get(&rule_id).unwrap();
    match &rule {
        Rule::Literal(c) => {
            if *c == msg.chars().next().unwrap() {
                &msg[1..]
            } else {
                msg
            }
        }
        Rule::Sub(first, second) => {
            let rest = match_concat(ruleset, &first, msg);
            if rest.len() < msg.len() {
                rest
            } else if let Some(second) = second {
                let rest = match_concat(ruleset, &second, msg);
                if rest.len() < msg.len() {
                    rest
                } else {
                    msg
                }
            } else {
                msg
            }
        }
    }
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let input_info = parse_input(input);
        let res = input_info
            .msgs
            .iter()
            .filter(|msg| {
                let rest = match_msg(&input_info.ruleset, 0, msg);
                rest.is_empty()
            })
            .count();
        format!("{}", res)
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
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let d = Day19 {};
        assert_eq!(d.star1(input), "2");
    }
}
