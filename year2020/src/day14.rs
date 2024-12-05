use common::day::Day;
use std::collections::HashMap;

pub struct Day14 {}

const ADDR_WIDTH: u64 = 36;

enum Cmd {
    Mask { mask0: u64, mask1: u64, float: u64 },
    MemSet { addr: u64, value: u64 },
}

fn parse_input(input: &str) -> Vec<Cmd> {
    let mut cmds = vec![];
    for line in input.lines() {
        let mut sp = line.split(" = ");
        let left = sp.next().unwrap();
        let right = sp.next().unwrap();
        if left == "mask" {
            let (mask0, mask1, float) = parse_mask_str(right);
            cmds.push(Cmd::Mask {
                mask0,
                mask1,
                float,
            });
        } else if left.starts_with("mem") {
            let sb = left.find('[').unwrap_or(0);
            let eb = left.find(']').unwrap_or(left.len());
            let addr = left[sb + 1..eb].parse::<u64>().unwrap();
            let value = right.parse::<u64>().unwrap();
            cmds.push(Cmd::MemSet { addr, value });
        } else {
            panic!("Invalid line: {}", line);
        }
    }
    cmds
}

fn parse_mask_str(mask_str: &str) -> (u64, u64, u64) {
    let mut digit = 1 << (ADDR_WIDTH - 1);
    let mut mask0 = 0xfffffffff;
    let mut mask1 = 0x0;
    let mut float = 0x0;
    for c in mask_str.chars() {
        match c {
            '0' => {
                mask0 ^= digit;
            }
            '1' => {
                mask1 ^= digit;
            }
            'X' => {
                float ^= digit;
            }
            _ => {}
        }
        digit >>= 1;
    }
    (mask0, mask1, float)
}

fn expand_addr(addr_in: u64, mask1: u64, float: u64) -> Vec<u64> {
    let mut addr_out = vec![addr_in | mask1];
    for digit in 0..ADDR_WIDTH {
        let digit = 1 << digit;
        if float & digit > 0 {
            let addr_toggle: Vec<_> = addr_out.iter().map(|x| x ^ digit).collect();
            addr_out.extend(addr_toggle);
        }
    }
    addr_out
}

fn run_cmds_part1(cmds: &[Cmd]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask0: u64 = 0xfffffffff;
    let mut mask1: u64 = 0;
    for cmd in cmds {
        match cmd {
            Cmd::Mask {
                mask0: new_mask0,
                mask1: new_mask1,
                float: _,
            } => {
                mask0 = *new_mask0;
                mask1 = *new_mask1;
            }
            Cmd::MemSet { addr, value } => {
                let m = mem.entry(*addr).or_insert(0);
                *m = value & mask0 | mask1;
            }
        }
    }
    mem.values().sum()
}

fn run_cmds_part2(cmds: &[Cmd]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask1: u64 = 0;
    let mut float: u64 = 0;
    for cmd in cmds {
        match cmd {
            Cmd::Mask {
                mask0: _,
                mask1: new_mask1,
                float: new_float,
            } => {
                mask1 = *new_mask1;
                float = *new_float;
            }
            Cmd::MemSet { addr, value } => {
                let addrs = expand_addr(*addr, mask1, float);
                for addr in addrs {
                    let m = mem.entry(addr).or_insert(0);
                    *m = *value;
                }
            }
        }
    }
    mem.values().sum()
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let sum = run_cmds_part1(&cmds);
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let sum = run_cmds_part2(&cmds);
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bitmask() {
        let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask0: u64 = 0b111111111111111111111111111111111101;
        let mask1: u64 = 0b000000000000000000000000000001000000;
        let float: u64 = 0b111111111111111111111111111110111101;

        assert_eq!(parse_mask_str(input), (mask0, mask1, float));
    }

    #[test]
    fn apply_bitmask() {
        let mask0: u64 = 0b1111111111111111111111111101;
        let mask1: u64 = 0b0000000000000000000001000000;

        assert_eq!(11 & mask0 | mask1, 73);
        assert_eq!(101 & mask0 | mask1, 101);
        assert_eq!(mask1, 64);
    }

    #[test]
    fn test_expand_addr1() {
        let mask1: u64 = 0b010010;
        let float: u64 = 0b100001;
        let addr: u64 = 0b101010;

        let mut found = expand_addr(addr, mask1, float);
        let mut expected = vec![26, 27, 58, 59];
        found.sort();
        expected.sort();

        assert_eq!(found, expected);
    }

    #[test]
    fn test_expand_addr2() {
        let mask1: u64 = 0;
        let float: u64 = 0b1011;
        let addr: u64 = 0b11010;

        let mut found = expand_addr(addr, mask1, float);
        let mut expected = vec![16, 17, 18, 19, 24, 25, 26, 27];
        found.sort();
        expected.sort();

        assert_eq!(found, expected);
    }

    #[test]
    fn ex1() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        let d = Day14 {};
        assert_eq!(d.star1(input), "165");
    }

    #[test]
    fn ex2() {
        let input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        let d = Day14 {};
        assert_eq!(d.star2(input), "208");
    }
}
