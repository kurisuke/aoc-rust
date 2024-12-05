use common::day::Day;

pub struct Day20 {}

type Interval = (u32, u32);

fn merge(mut input: Vec<Interval>) -> Vec<Interval> {
    input.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by max first
    let mut output = vec![input.pop().unwrap()];
    while !input.is_empty() && output.last().unwrap().1 != u32::MAX {
        let next = input.pop().unwrap();
        let top = output.pop().unwrap();
        if next.0 <= top.1 + 1 {
            output.push((top.0, top.1.max(next.1)));
        } else {
            output.push(top);
            output.push(next);
        }
    }
    output
}

fn parse_input(input: &str) -> Vec<Interval> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split('-').collect();
            (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
            )
        })
        .collect()
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let intervals = parse_input(input);
        let merged = merge(intervals);
        // assume first interval starts with 0 (from the input data)
        format!("{}", merged[0].1 + 1)
    }

    fn star2(&self, input: &str) -> String {
        let intervals = parse_input(input);
        let merged = merge(intervals);

        // assume first interval starts with 0 and last intervall ends with u32::MAX
        // (from the input data)
        let allowed: u32 = merged.windows(2).map(|w| w[1].0 - w[0].1 - 1).sum();
        format!("{}", allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day20 {};
        let input = r#"5-8
0-2
4-7"#;
        assert_eq!(d.star1(input), "3");
    }
}
