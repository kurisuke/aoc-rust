use std::collections::{HashMap, VecDeque};

use common::day::Day;

pub struct Day19 {}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let (workflows, parts) = parse_input(input);
        parts
            .iter()
            .filter(|part| part.accepted(&workflows))
            .map(|part| part.total())
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let (workflows, _) = parse_input(input);
        let accepted_ranges = accepted_ranges(&workflows);
        accepted_ranges
            .iter()
            .map(|ranges| {
                ranges
                    .iter()
                    .map(|range| range.1 - range.0)
                    .product::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let secs: Vec<_> = input.split("\n\n").collect();
    let workflows = secs[0]
        .lines()
        .map(Workflow::parse)
        .map(|w| (w.name, w))
        .collect();
    let parts = secs[1].lines().map(Part::parse).collect();
    (workflows, parts)
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: Target<'a>,
}

impl Workflow<'_> {
    fn parse(line: &str) -> Workflow {
        let spl: Vec<_> = line.split('{').collect();
        let name = spl[0];

        let rules_str = &spl[1][0..spl[1].len() - 1];
        let mut rules_str: Vec<_> = rules_str.split(',').collect();
        let default = Target::parse(rules_str.pop().unwrap());

        let rules = rules_str
            .iter()
            .map(|r| {
                let idx = match r.chars().next().unwrap() {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                let greater = match r.chars().nth(1).unwrap() {
                    '>' => true,
                    '<' => false,
                    _ => unreachable!(),
                };
                let spl: Vec<_> = (r[2..]).split(':').collect();
                let val = spl[0].parse().unwrap();
                let target = Target::parse(spl[1]);
                Rule {
                    idx,
                    greater,
                    val,
                    target,
                }
            })
            .collect();

        Workflow {
            name,
            rules,
            default,
        }
    }
}

struct Rule<'a> {
    idx: usize,
    greater: bool,
    val: usize,
    target: Target<'a>,
}

enum Target<'a> {
    Accepted,
    Rejected,
    Other(&'a str),
}

impl Target<'_> {
    fn parse(s: &str) -> Target {
        match s {
            "A" => Target::Accepted,
            "R" => Target::Rejected,
            _ => Target::Other(s),
        }
    }
}

struct Part {
    ratings: [usize; 4],
}

impl Part {
    fn parse(line: &str) -> Part {
        let line = &line[1..line.len() - 1];
        Part {
            ratings: line
                .split(',')
                .map(|s| s.split('=').nth(1).unwrap().parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let mut next_workflow_id = None;

            for rule in &workflow.rules {
                let fulfilled = if rule.greater {
                    self.ratings[rule.idx] > rule.val
                } else {
                    self.ratings[rule.idx] < rule.val
                };
                if fulfilled {
                    match rule.target {
                        Target::Accepted => {
                            return true;
                        }
                        Target::Rejected => {
                            return false;
                        }
                        Target::Other(workflow_id) => {
                            next_workflow_id = Some(workflow_id);
                            break;
                        }
                    }
                }
            }

            if let Some(workflow_id) = next_workflow_id {
                workflow = workflows.get(workflow_id).unwrap();
            } else {
                match workflow.default {
                    Target::Accepted => {
                        return true;
                    }
                    Target::Rejected => {
                        return false;
                    }
                    Target::Other(workflow_id) => {
                        workflow = workflows.get(workflow_id).unwrap();
                    }
                }
            }
        }
    }

    fn total(&self) -> usize {
        self.ratings.iter().sum()
    }
}

// part 2

type Ranges = [(usize, usize); 4];

struct SearchState<'a> {
    node: &'a str,
    ranges: Ranges,
}

fn accepted_ranges(workflows: &HashMap<&str, Workflow>) -> Vec<Ranges> {
    let mut accepted = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(SearchState {
        node: "in",
        ranges: [(1, 4001), (1, 4001), (1, 4001), (1, 4001)],
    });

    while let Some(mut cur) = queue.pop_front() {
        let workflow = workflows.get(cur.node).unwrap();
        for rule in &workflow.rules {
            let mut ranges_new = cur.ranges;
            if rule.greater {
                ranges_new[rule.idx].0 = rule.val + 1;
            } else {
                ranges_new[rule.idx].1 = rule.val;
            }

            if ranges_new[rule.idx].0 >= ranges_new[rule.idx].1 {
                // empty range, skip
            } else {
                if rule.greater {
                    cur.ranges[rule.idx].1 = ranges_new[rule.idx].0;
                } else {
                    cur.ranges[rule.idx].0 = ranges_new[rule.idx].1;
                }

                match rule.target {
                    Target::Accepted => accepted.push(ranges_new),
                    Target::Rejected => {}
                    Target::Other(node) => {
                        queue.push_back(SearchState {
                            node,
                            ranges: ranges_new,
                        });
                    }
                }
            }

            // check if remaining is empty
            if cur.ranges.iter().any(|range| range.0 >= range.1) {
                break;
            }
        }

        if cur.ranges.iter().all(|range| range.0 < range.1) {
            match workflow.default {
                Target::Accepted => accepted.push(cur.ranges),
                Target::Rejected => {}
                Target::Other(node) => {
                    queue.push_back(SearchState {
                        node,
                        ranges: cur.ranges,
                    });
                }
            }
        }
    }

    accepted
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn ex1() {
        let d = Day19 {};
        assert_eq!(d.star1(INPUT), "19114");
    }

    #[test]
    fn ex2() {
        let d = Day19 {};
        assert_eq!(d.star2(INPUT), "167409079868000");
    }
}
