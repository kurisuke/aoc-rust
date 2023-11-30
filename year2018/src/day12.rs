use common::day::Day;
use std::collections::BTreeSet;
use std::collections::HashMap;

pub struct Day12 {}

type NeighborConfig = [bool; 5];
type Rules = HashMap<NeighborConfig, bool>;
type Gen = BTreeSet<isize>;

fn next_val(pos: isize, last_gen: &Gen, rules: &Rules) -> bool {
    let mut config = [false; 5];
    for i in -2..=2 {
        config[(i + 2) as usize] = last_gen.contains(&(pos + i));
    }
    *rules.get(&config).unwrap()
}

fn next_gen(last_gen: &Gen, rules: &Rules) -> Gen {
    let mut next_gen = BTreeSet::new();
    let min_pos = last_gen.iter().next().unwrap() - 2;
    let max_pos = last_gen.iter().next_back().unwrap() + 2;
    for pos in min_pos..=max_pos {
        if next_val(pos, last_gen, rules) {
            next_gen.insert(pos);
        }
    }
    next_gen
}

fn parse_input(input: &str) -> (Gen, Rules) {
    let mut it_parts = input.split("\n\n");

    // initial state
    let isline = it_parts.next().unwrap();
    let isline_parts: Vec<_> = isline.split_whitespace().collect();
    let mut init_gen = BTreeSet::new();
    for (i, c) in isline_parts[2].chars().enumerate() {
        if c == '#' {
            init_gen.insert(i as isize);
        }
    }

    // rules
    let rules_lines = it_parts.next().unwrap();
    let mut rules = HashMap::new();
    for line in rules_lines.lines() {
        let line_parts: Vec<_> = line.split_whitespace().collect();
        let mut config = [false; 5];
        for (i, item) in config.iter_mut().enumerate() {
            *item = line_parts[0].chars().nth(i).unwrap() == '#';
        }
        rules.insert(config, line_parts[2].starts_with('#'));
    }

    (init_gen, rules)
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let (mut gen, rules) = parse_input(input);
        for _ in 0..20 {
            gen = next_gen(&gen, &rules);
        }
        format!("{}", gen.iter().sum::<isize>())
    }

    fn star2(&self, input: &str) -> String {
        let (mut gen, rules) = parse_input(input);

        let mut sum_1000 = 0;
        for i in 1..=2000 {
            gen = next_gen(&gen, &rules);
            if i == 1000 {
                sum_1000 = gen.iter().sum::<isize>();
            }
        }
        let sum_2000 = gen.iter().sum::<isize>();
        format!("{}", sum_1000 + (sum_2000 - sum_1000) * (49_999_999))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"initial state: #..#.#..##......###...###

##### => .
####. => #
###.# => #
###.. => #
##.## => #
##.#. => #
##..# => .
##... => .
#.### => #
#.##. => .
#.#.# => #
#.#.. => .
#..## => .
#..#. => .
#...# => .
#.... => .
.#### => #
.###. => .
.##.# => .
.##.. => #
.#.## => #
.#.#. => #
.#..# => .
.#... => #
..### => .
..##. => .
..#.# => .
..#.. => #
...## => #
...#. => .
....# => .
..... => ."#;

        let d = Day12 {};
        assert_eq!(d.star1(input), "325");
    }
}
