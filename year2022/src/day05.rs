use common::day::Day;

pub struct Day05 {}

type Crate = char;

type Stack = Vec<Crate>;

struct MoveCmd {
    amount: usize,
    from_stack: usize,
    to_stack: usize,
}

impl MoveCmd {
    fn apply(&self, stacks: &mut [Stack]) {
        for _ in 0..self.amount {
            if let Some(c) = stacks[self.from_stack].pop() {
                stacks[self.to_stack].push(c);
            }
        }
    }

    fn apply_pt2(&self, stacks: &mut [Stack]) {
        let split_idx = stacks[self.from_stack].len() - self.amount;
        let mut move_crates = stacks[self.from_stack].split_off(split_idx);
        stacks[self.to_stack].append(&mut move_crates);
    }
}

fn parse_init_stacks(input: &str) -> Vec<Stack> {
    let lines: Vec<_> = input.lines().rev().collect();

    let num_stacks = lines[0].split_whitespace().count();

    let mut stacks = vec![vec![]; num_stacks];

    for line in lines.iter().skip(1) {
        let line_chars: Vec<_> = line.chars().collect();
        for (idx_stack, stack) in stacks.iter_mut().enumerate() {
            let col = 1 + 4 * idx_stack;
            if line_chars[col] != ' ' {
                stack.push(line_chars[col]);
            }
        }
    }

    stacks
}

fn parse_move_cmds(input: &str) -> impl Iterator<Item = MoveCmd> + '_ {
    input.lines().map(|line| {
        let tokens: Vec<_> = line.split_whitespace().collect();
        MoveCmd {
            amount: tokens[1].parse().unwrap(),
            from_stack: tokens[3].parse::<usize>().unwrap() - 1,
            to_stack: tokens[5].parse::<usize>().unwrap() - 1,
        }
    })
}

fn parse_input(input: &str) -> (Vec<Stack>, impl Iterator<Item = MoveCmd> + '_) {
    let mut iter = input.split("\n\n");
    (
        parse_init_stacks(iter.next().unwrap()),
        parse_move_cmds(iter.next().unwrap()),
    )
}

fn run(mut stacks: Vec<Stack>, move_cmds: impl Iterator<Item = MoveCmd>, part2: bool) -> String {
    if !part2 {
        for move_cmd in move_cmds {
            move_cmd.apply(&mut stacks);
        }
    } else {
        for move_cmd in move_cmds {
            move_cmd.apply_pt2(&mut stacks);
        }
    }

    stacks.iter().filter_map(|s| s.last()).collect()
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let (stacks, move_cmds) = parse_input(input);
        run(stacks, move_cmds, false)
    }

    fn star2(&self, input: &str) -> String {
        let (stacks, move_cmds) = parse_input(input);
        run(stacks, move_cmds, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let d = Day05 {};
        assert_eq!(d.star1(input), "CMZ");
        assert_eq!(d.star2(input), "MCD");
    }
}
