use common::day::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let end_pos = cmds
            .iter()
            .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));
        format!("{}", end_pos.0 * end_pos.1)
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut aim = 0i64;
        let mut end_pos = (0i64, 0i64);
        for cmd in cmds {
            aim += cmd.1;
            end_pos = (end_pos.0 + cmd.0, end_pos.1 + aim * cmd.0);
        }
        format!("{}", end_pos.0 * end_pos.1)
    }
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let tokens: Vec<_> = l.split(' ').collect();
            if tokens.len() > 1 {
                let amount = tokens[1].parse::<i64>().unwrap();
                match tokens[0] {
                    "forward" => Ok((amount, 0)),
                    "up" => Ok((0, -amount)),
                    "down" => Ok((0, amount)),
                    _ => Err((0, 0)),
                }
            } else {
                Err((0, 0))
            }
        })
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

        let d = Day02 {};
        assert_eq!(d.star1(input), "150");
        assert_eq!(d.star2(input), "900");
    }
}
