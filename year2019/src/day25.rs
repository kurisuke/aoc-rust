use common::day::Day;
use util::intcode::{Intcode, RunState};

const INIT_CMDS: &str = r#"north
take tambourine
north
east
west
south
east
take astrolabe
east
north
take klein bottle
north
take easter egg
south
south
west
south
take shell
north
west
south
south
south
take hypercube
north
west
east
north
west
take dark matter
west
north
east
east
west
west
west
take coin
south"#;

pub struct Day25 {}

fn get_inv(intcode: &mut Intcode) -> Vec<String> {
    intcode.write_inp_ascii("inv\n");
    intcode.run();
    let mut items = vec![];
    for line in intcode.read_outp_ascii().lines() {
        if line.starts_with("- ") && line != "- north" && line != "- south" {
            items.push(line[2..].to_string());
        }
    }
    items
}

fn num_to_bits(mut n: usize, l: usize) -> Vec<bool> {
    let mut v = vec![];
    for _ in 0..l {
        v.push(n & 1 > 0);
        n /= 2;
    }
    v.into_iter().rev().collect()
}

fn parse_code(desc: &str) -> usize {
    let num_str = desc
        .split_whitespace()
        .find(|w| w.chars().all(|c| c.is_numeric()))
        .unwrap();
    num_str.parse().unwrap()
}

fn try_items(intcode: &mut Intcode) -> usize {
    // get initial inventory (all items)
    let all_items = get_inv(intcode);
    let mut last_inv: Option<Vec<bool>> = None;

    for i in (1..(1 << all_items.len())).rev() {
        let cur_inv = num_to_bits(i, all_items.len());

        let mut cmd_list = vec![];
        for (j, _) in cur_inv.iter().enumerate() {
            if let Some(last_inv) = last_inv.clone() {
                if cur_inv[j] && !last_inv[j] {
                    cmd_list.push(format!("take {}\n", all_items[j]));
                } else if !cur_inv[j] && last_inv[j] {
                    cmd_list.push(format!("drop {}\n", all_items[j]));
                }
            }
        }

        // run the take / drop commands until the inventory is okay
        for cmd in cmd_list {
            intcode.write_inp_ascii(&cmd);
            intcode.run();
            intcode.read_outp_ascii(); // ignore
        }

        // then go south and try
        intcode.write_inp_ascii("south\n");
        intcode.run();
        let desc = intcode.read_outp_ascii();

        if !desc.contains("ejected") {
            return parse_code(&desc);
        } else {
            last_inv = Some(cur_inv);
        }
    }
    0
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        for cmd in INIT_CMDS.lines() {
            intcode.run();
            if intcode.state == RunState::Halted {
                break;
            }
            let _desc = intcode.read_outp_ascii();
            // print!("{}", desc);
            // println!("? {}", cmd);
            intcode.write_inp_ascii(cmd);
            intcode.write_inp_ascii("\n");
        }
        format!("{}", try_items(&mut intcode))
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}
