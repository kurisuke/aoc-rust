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

fn match_seq<'a>(ruleset: &Ruleset, seq: &[usize], msg: &'a str) -> Vec<&'a str> {
    if seq.is_empty() {
        vec![msg]
    } else {
        match_msg(ruleset, seq[0], msg)
            .iter()
            .map(|rest| match_seq(ruleset, &seq[1..], rest))
            .flatten()
            .collect()
    }
}

fn match_alt<'a>(
    ruleset: &Ruleset,
    alt1: &[usize],
    alt2: &Option<Vec<usize>>,
    msg: &'a str,
) -> Vec<&'a str> {
    let mut ret = match_seq(ruleset, alt1, msg);
    if let Some(alt2) = alt2 {
        ret.extend(match_seq(ruleset, alt2, msg));
    }
    ret
}

fn match_msg<'a>(ruleset: &Ruleset, rule_id: usize, msg: &'a str) -> Vec<&'a str> {
    let rule = ruleset
        .get(&rule_id)
        .unwrap_or_else(|| panic!("unknown rule: {}", rule_id));
    match &rule {
        Rule::Literal(c) => {
            if !msg.is_empty() && *c == msg.chars().next().unwrap() {
                vec![&msg[1..]]
            } else {
                vec![]
            }
        }
        Rule::Sub(alt1, alt2) => match_alt(ruleset, alt1, alt2, msg),
    }
}

fn get_result(input_info: InputInfo) -> usize {
    input_info
        .msgs
        .iter()
        .filter(|msg| {
            let ms = match_msg(&input_info.ruleset, 0, msg);
            ms.iter().any(|x| x.is_empty())
        })
        .count()
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        format!("{}", get_result(parse_input(input)))
    }

    fn star2(&self, input: &str) -> String {
        let mut input_info = parse_input(input);
        input_info
            .ruleset
            .entry(8)
            .and_modify(|e| *e = Rule::Sub(vec![42], Some(vec![42, 8])));
        input_info
            .ruleset
            .entry(11)
            .and_modify(|e| *e = Rule::Sub(vec![42, 31], Some(vec![42, 11, 31])));
        format!("{}", get_result(input_info))
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

    #[test]
    fn ex2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let d = Day19 {};
        assert_eq!(d.star1(input), "3");
        assert_eq!(d.star2(input), "12");
    }
}
