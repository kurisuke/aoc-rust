use common::day::Day;

use std::collections::{HashMap, HashSet};

const DIGITS_ORIG: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

struct Display {
    digits: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

pub struct Day08 {}

fn parse_input(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|l| {
            let spl: Vec<_> = l.split(" | ").collect();
            let digits = spl[0].split(' ').map(chars_to_set).collect();
            let output = spl[1].split(' ').map(chars_to_set).collect();
            Display { digits, output }
        })
        .collect()
}

fn chars_to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn resolve_mapping(digits: &[HashSet<char>]) -> HashMap<char, char> {
    let mut ret = HashMap::new();

    // digit 1 (2 segments)
    // 2 segments are either c or f
    let cand_cf = digits.iter().find(|d| d.len() == 2).unwrap();

    // digit 7 (3 segments)
    // the segment not in 1 (cand_cf) is the "top" segment (a)
    let l3 = digits.iter().find(|d| d.len() == 3).unwrap();
    let a = *l3.difference(cand_cf).next().unwrap();
    ret.insert(a, 'a');

    // digit 4 (4 segments)
    // segments not in cand_cf are b & d
    let l4 = digits.iter().find(|d| d.len() == 4).unwrap();
    let cand_bd: HashSet<_> = l4.difference(cand_cf).cloned().collect();

    // remaining segments
    let l7 = digits.iter().find(|d| d.len() == 7).unwrap();
    let mut others: HashSet<_> = cand_cf.union(&cand_bd).cloned().collect();
    others.insert(a);
    let cand_eg: HashSet<_> = l7.difference(&others).cloned().collect();

    for l6 in digits.iter().filter(|d| d.len() == 6) {
        let diff = *l7.difference(l6).next().unwrap();

        // digit 6
        if cand_cf.contains(&diff) {
            ret.insert(diff, 'c');
            let diff_set: HashSet<_> = [diff].iter().cloned().collect();
            let other = *cand_cf.difference(&diff_set).next().unwrap();
            ret.insert(other, 'f');
        // digit 9
        } else if cand_eg.contains(&diff) {
            ret.insert(diff, 'e');
            let diff_set: HashSet<_> = [diff].iter().cloned().collect();
            let other = *cand_eg.difference(&diff_set).next().unwrap();
            ret.insert(other, 'g');
        // digit 0
        } else if cand_bd.contains(&diff) {
            ret.insert(diff, 'd');
            let diff_set: HashSet<_> = [diff].iter().cloned().collect();
            let other = *cand_bd.difference(&diff_set).next().unwrap();
            ret.insert(other, 'b');
        }
    }

    ret
}

fn unscramble(
    scrambled: &HashSet<char>,
    mapping: &HashMap<char, char>,
    orig: &[HashSet<char>],
) -> Option<usize> {
    let mapped: HashSet<_> = scrambled
        .iter()
        .map(|x| mapping.get(x).unwrap())
        .cloned()
        .collect();
    for (i, o) in orig.iter().enumerate() {
        if &mapped == o {
            return Some(i);
        }
    }
    None
}

fn decode_number(display: &Display, orig: &[HashSet<char>]) -> usize {
    let mapping = resolve_mapping(&display.digits);
    let mut s = 0;
    for (exp, n) in display.output.iter().enumerate() {
        let u = unscramble(n, &mapping, orig).unwrap();
        s += u * 10_usize.pow(3 - exp as u32);
    }
    s
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let displays = parse_input(input);
        let sum_digits = displays
            .iter()
            .map(|d| {
                d.output
                    .iter()
                    .filter(|n| n.len() != 5 && n.len() != 6)
                    .count()
            })
            .sum::<usize>();
        format!("{}", sum_digits)
    }

    fn star2(&self, input: &str) -> String {
        let displays = parse_input(input);
        let orig: Vec<HashSet<_>> = DIGITS_ORIG.iter().map(|d| d.chars().collect()).collect();
        let sum: usize = displays.iter().map(|d| decode_number(d, &orig)).sum();
        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input1 = r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#;

        let input2 = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        let d = Day08 {};
        assert_eq!(d.star1(input1), "0");
        assert_eq!(d.star1(input2), "26");
        assert_eq!(d.star2(input1), "5353");
        assert_eq!(d.star2(input2), "61229");
    }
}
