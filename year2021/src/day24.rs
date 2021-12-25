use common::day::Day;

pub struct Day24 {}

struct BlockParams {
    div_z: i64,
    add_x: i64,
    add_y: i64,
}

struct DigitRule {
    target: usize,
    val: i64,
}

fn parse_input(input: &str) -> Vec<BlockParams> {
    let mut block_params = vec![];
    let lines: Vec<_> = input.lines().collect();
    for block in lines.chunks(18) {
        block_params.push(BlockParams {
            div_z: block[4].split(' ').nth(2).unwrap().parse().unwrap(),
            add_x: block[5].split(' ').nth(2).unwrap().parse().unwrap(),
            add_y: block[15].split(' ').nth(2).unwrap().parse().unwrap(),
        });
    }
    block_params
}

fn digit_rules(block_params: &[BlockParams]) -> [i64; 14] {
    let mut stack: Vec<DigitRule> = vec![];
    let mut dr = [0; 14];
    for (i, b) in block_params.iter().enumerate() {
        match b.div_z {
            26 => {
                let s = stack.pop().unwrap();
                dr[i] = s.val + b.add_x;
                dr[s.target] = -s.val - b.add_x;
            }
            1 => {
                stack.push(DigitRule {
                    target: i,
                    val: b.add_y,
                });
            }
            _ => {
                unreachable!();
            }
        }
    }
    dr
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let block_params = parse_input(input);
        let rules = digit_rules(&block_params);
        rules.map(|r| 9.min(9 + r).to_string()).concat()
    }

    fn star2(&self, input: &str) -> String {
        let block_params = parse_input(input);
        let rules = digit_rules(&block_params);
        rules.map(|r| 1.max(1 + r).to_string()).concat()
    }
}
