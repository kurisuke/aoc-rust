use bitvec::prelude::*;
use common::day::Day;

pub struct Day16 {}

fn parse_input(input: &str) -> BitVec {
    let len = input.trim().len();
    let mut i = 0;
    let mut nums = vec![];
    while i < len {
        nums.push(u8::from_str_radix(&input[i..i + 2], 16).unwrap());
        i += 2;
    }
    nums.view_bits::<Msb0>().iter().collect()
}

fn to_value(bits: &BitSlice) -> usize {
    let v: BitVec = bits.iter().rev().collect();
    v.load::<usize>()
}

fn parse_packet(bits: &BitSlice) -> (usize, usize, usize) {
    let mut version = to_value(&bits[0..3]);
    let typ = to_value(&bits[3..6]);
    let mut cursor = 0;

    let val = match typ {
        4 => {
            let (literal, bits_read) = parse_number(&bits[6..]);
            cursor += 6 + bits_read;
            literal
        }
        _ => {
            let length_id = bits[6];
            let mut subpacket_vals = vec![];
            match length_id {
                true => {
                    let num_subpackets = to_value(&bits[7..18]);
                    cursor = 18;
                    for _ in 0..num_subpackets {
                        let (version_sub, bits_read, val_sub) = parse_packet(&bits[cursor..]);
                        subpacket_vals.push(val_sub);
                        version += version_sub;
                        cursor += bits_read;
                    }
                }
                false => {
                    let total_length = to_value(&bits[7..22]);
                    cursor = 22;
                    while cursor - 22 < total_length {
                        let (version_sub, bits_read, val_sub) = parse_packet(&bits[cursor..]);
                        subpacket_vals.push(val_sub);
                        version += version_sub;
                        cursor += bits_read;
                    }
                }
            }
            eval(typ, &subpacket_vals)
        }
    };
    (version, cursor, val)
}

fn parse_number(bits: &BitSlice) -> (usize, usize) {
    let mut cursor = 0;
    let mut number_bits = BitVec::new();
    loop {
        let start = bits[cursor];
        number_bits.extend_from_bitslice(&bits[cursor + 1..cursor + 5]);
        cursor += 5;
        if !start {
            break;
        }
    }
    (to_value(&number_bits), cursor)
}

fn eval(typ: usize, vals: &[usize]) -> usize {
    match typ {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => {
            if vals[0] > vals[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if vals[0] < vals[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if vals[0] == vals[1] {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("unknown type: {}", typ);
        }
    }
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let bits = parse_input(input);
        let (version, _, _) = parse_packet(&bits);
        format!("{}", version)
    }

    fn star2(&self, input: &str) -> String {
        let bits = parse_input(input);
        let (_, _, val) = parse_packet(&bits);
        format!("{}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec2str(v: &BitVec) -> String {
        let mut s = String::new();
        for c in v {
            if *c {
                s = format!("{}1", s);
            } else {
                s = format!("{}0", s);
            }
        }
        s
    }

    #[test]
    fn test_parse() {
        assert_eq!(vec2str(&parse_input("D2FE28")), "110100101111111000101000");
        assert_eq!(
            vec2str(&parse_input("38006F45291200")),
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            vec2str(&parse_input("EE00D40C823060")),
            "11101110000000001101010000001100100000100011000001100000"
        );
    }

    #[test]
    fn star1() {
        let d = Day16 {};
        assert_eq!(d.star1("8A004A801A8002F478"), "16");
        assert_eq!(d.star1("620080001611562C8802118E34"), "12");
        assert_eq!(d.star1("C0015000016115A2E0802F182340"), "23");
        assert_eq!(d.star1("A0016C880162017C3686B18A3D4780"), "31");
    }

    #[test]
    fn star2() {
        let d = Day16 {};
        assert_eq!(d.star2("C200B40A82"), "3");
        assert_eq!(d.star2("04005AC33890"), "54");
        assert_eq!(d.star2("880086C3E88112"), "7");
        assert_eq!(d.star2("CE00C43D881120"), "9");
        assert_eq!(d.star2("D8005AC2A8F0"), "1");
        assert_eq!(d.star2("F600BC2D8F"), "0");
        assert_eq!(d.star2("9C005AC2F8F0"), "0");
        assert_eq!(d.star2("9C0141080250320F1802104A08"), "1");
    }
}
