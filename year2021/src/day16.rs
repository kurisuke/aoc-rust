use bitvec::prelude::*;
use common::day::Day;

pub struct Day16 {}

struct PacketEval {
    version_sum: usize,
    val: usize,
}

struct BitReader {
    bits: BitVec<Msb0, u8>,
    pos: usize,
}

impl BitReader {
    fn slice(&mut self, k: usize) -> &BitSlice<Msb0, u8> {
        let r = &self.bits[self.pos..self.pos + k];
        self.pos += k;
        r
    }
    fn bit(&mut self) -> bool {
        let r = self.bits[self.pos];
        self.pos += 1;
        r
    }
    fn load(&mut self, k: usize) -> usize {
        self.slice(k).load_be()
    }
}

fn parse_input(input: &str) -> BitReader {
    let len = input.trim().len();
    let nums = (0..len)
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect();
    BitReader {
        bits: BitVec::from_vec(nums),
        pos: 0,
    }
}

fn parse_packet(r: &mut BitReader) -> PacketEval {
    let mut version_sum = r.load(3);
    let typ = r.load(3);

    let val = match typ {
        4 => parse_literal(r),
        _ => {
            let length_id = r.bit();
            let mut subpacket_vals = vec![];
            match length_id {
                true => {
                    let num_subpackets: usize = r.load(11);
                    for _ in 0..num_subpackets {
                        let sub = parse_packet(r);
                        subpacket_vals.push(sub.val);
                        version_sum += sub.version_sum;
                    }
                }
                false => {
                    let total_length = r.load(15);
                    let target = r.pos + total_length;
                    while r.pos < target {
                        let sub = parse_packet(r);
                        subpacket_vals.push(sub.val);
                        version_sum += sub.version_sum;
                    }
                }
            }
            eval(typ, &subpacket_vals)
        }
    };
    PacketEval { version_sum, val }
}

fn parse_literal(r: &mut BitReader) -> usize {
    let mut number_bits = BitVec::<Msb0, u8>::new();
    loop {
        let start = r.bit();
        number_bits.extend(r.slice(4));
        if !start {
            break;
        }
    }
    number_bits.load_be()
}

fn eval(typ: usize, vals: &[usize]) -> usize {
    match typ {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => (vals[0] > vals[1]) as usize,
        6 => (vals[0] < vals[1]) as usize,
        7 => (vals[0] == vals[1]) as usize,
        _ => unreachable!(),
    }
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let mut bit_reader = parse_input(input);
        let packet = parse_packet(&mut bit_reader);
        format!("{}", packet.version_sum)
    }

    fn star2(&self, input: &str) -> String {
        let mut bit_reader = parse_input(input);
        let packet = parse_packet(&mut bit_reader);
        format!("{}", packet.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec2str(r: &mut BitReader) -> String {
        let mut s = String::new();
        for c in r.bits.iter() {
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
        assert_eq!(
            vec2str(&mut parse_input("D2FE28")),
            "110100101111111000101000"
        );
        assert_eq!(
            vec2str(&mut parse_input("38006F45291200")),
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            vec2str(&mut parse_input("EE00D40C823060")),
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
