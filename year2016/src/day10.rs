use common::day::Day;
use std::collections::BTreeMap;

pub struct Day10 {}

#[derive(Copy, Clone, Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

struct BotConfig {
    target_lo: Target,
    target_hi: Target,
}

struct Bot {
    config: BotConfig,
    chips: Vec<usize>,
}

type Outputs = Vec<Option<usize>>;

fn run_transfers(bots: &mut [Bot], outputs: &mut Outputs, transfers: &[usize]) {
    for src_id in transfers {
        let (val_lo, val_hi) = if bots[*src_id].chips[0] < bots[*src_id].chips[1] {
            (bots[*src_id].chips[0], bots[*src_id].chips[1])
        } else {
            (bots[*src_id].chips[1], bots[*src_id].chips[0])
        };
        match bots[*src_id].config.target_lo {
            Target::Bot(target_id) => {
                bots[target_id].chips.push(val_lo);
            }
            Target::Output(output_id) => {
                outputs[output_id] = Some(val_lo);
            }
        }
        match bots[*src_id].config.target_hi {
            Target::Bot(target_id) => {
                bots[target_id].chips.push(val_hi);
            }
            Target::Output(output_id) => {
                outputs[output_id] = Some(val_hi);
            }
        }
        bots[*src_id].chips.clear();
    }
}

fn get_transfers(bots: &[Bot], outputs: &[Option<usize>]) -> Vec<usize> {
    let mut transfers = vec![];
    for (src_id, bot) in bots.iter().enumerate().filter(|(_, b)| b.chips.len() == 2) {
        let mut can_transfer = true;
        // transfer lo
        match bot.config.target_lo {
            Target::Bot(target_id) => {
                if bots[target_id].chips.len() == 2 {
                    can_transfer = false;
                }
            }
            Target::Output(output_id) => {
                if outputs[output_id].is_some() {
                    can_transfer = false;
                }
            }
        }

        // transfer hi
        match bot.config.target_hi {
            Target::Bot(target_id) => {
                if bots[target_id].chips.len() == 2 {
                    can_transfer = false;
                }
            }
            Target::Output(output_id) => {
                if outputs[output_id].is_some() {
                    can_transfer = false;
                }
            }
        }

        if can_transfer {
            transfers.push(src_id)
        }
    }
    transfers
}

fn parse_input(input: &str) -> (Vec<Bot>, Outputs) {
    let mut bots = BTreeMap::new();
    let mut max_output = 0;
    let mut init_values = vec![];

    for line in input.lines() {
        if line.starts_with("bot") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let src_id = parts[1].parse::<usize>().unwrap();
            let lo_target_str = parts[5];
            let lo_target_val = parts[6].parse::<usize>().unwrap();
            let hi_target_str = parts[10];
            let hi_target_val = parts[11].parse::<usize>().unwrap();
            let target_lo = if lo_target_str == "bot" {
                Target::Bot(lo_target_val)
            } else {
                Target::Output(lo_target_val)
            };
            let target_hi = if hi_target_str == "bot" {
                Target::Bot(hi_target_val)
            } else {
                Target::Output(hi_target_val)
            };
            if lo_target_str == "output" {
                max_output = max_output.max(lo_target_val);
            }
            if hi_target_str == "output" {
                max_output = max_output.max(hi_target_val);
            }
            bots.insert(
                src_id,
                Bot {
                    config: BotConfig {
                        target_lo,
                        target_hi,
                    },
                    chips: vec![],
                },
            );
        } else if line.starts_with("value") {
            let parts: Vec<_> = line.split_whitespace().collect();
            init_values.push((
                parts[5].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            ));
        }
    }
    let mut bots: Vec<_> = bots.into_iter().map(|(_, v)| v).collect();
    for v in init_values {
        bots[v.0].chips.push(v.1);
    }
    let outputs = vec![None; max_output + 1];

    (bots, outputs)
}

fn find_responsible(input: &str, v1: usize, v2: usize) -> usize {
    let (mut bots, mut outputs) = parse_input(input);
    while outputs.iter().any(|x| x.is_none()) {
        let bot_to_find: Vec<_> = bots
            .iter()
            .enumerate()
            .filter(|(_, b)| b.chips.contains(&v1) && b.chips.contains(&v2))
            .collect();
        if !bot_to_find.is_empty() {
            return bot_to_find[0].0;
        } else {
            let transfers = get_transfers(&bots, &outputs);
            run_transfers(&mut bots, &mut outputs, &transfers);
        }
    }
    0
}

fn get_outputs(input: &str) -> Outputs {
    let (mut bots, mut outputs) = parse_input(input);
    while outputs.iter().any(|x| x.is_none()) {
        let transfers = get_transfers(&bots, &outputs);
        run_transfers(&mut bots, &mut outputs, &transfers);
    }
    outputs
}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        format!("{}", find_responsible(input, 17, 61))
    }

    fn star2(&self, input: &str) -> String {
        let outputs = get_outputs(input);
        let prod = outputs[0].unwrap() * outputs[1].unwrap() * outputs[2].unwrap();
        format!("{}", prod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#;
        assert_eq!(find_responsible(input, 5, 2), 2);
    }
}
