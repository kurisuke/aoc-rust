use common::day::Day;

use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Day11 {}

enum MonkeyOp {
    Add(usize),
    Mul(usize),
    AddSelf,
    MulSelf,
}

struct Monkey {
    pub items: VecDeque<usize>,
    op: MonkeyOp,
    pub test_divisor: usize,
    target_true: usize,
    target_false: usize,

    pub inspected_items: usize,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let mut lines = input.lines().filter_map(|line| line.split(": ").nth(1));

        // Starting items
        let item_str = lines.next().unwrap();
        let items = item_str.split(", ").map(|i| i.parse().unwrap()).collect();

        // Operation
        let op_str = lines.next().unwrap();
        let tokens: Vec<_> = op_str.split_whitespace().collect();
        let op = match tokens[3] {
            "*" => {
                if tokens[4] == "old" {
                    MonkeyOp::MulSelf
                } else {
                    MonkeyOp::Mul(tokens[4].parse().unwrap())
                }
            }
            "+" => {
                if tokens[4] == "old" {
                    MonkeyOp::AddSelf
                } else {
                    MonkeyOp::Add(tokens[4].parse().unwrap())
                }
            }
            _ => unreachable!(),
        };

        // Test divisor
        let test_divisor_str = lines.next().unwrap();
        let tokens: Vec<_> = test_divisor_str.split_whitespace().collect();
        let test_divisor = tokens[2].parse().unwrap();

        // Targets
        let target_str = lines.next().unwrap();
        let tokens: Vec<_> = target_str.split_whitespace().collect();
        let target_true = tokens[3].parse().unwrap();

        let target_str = lines.next().unwrap();
        let tokens: Vec<_> = target_str.split_whitespace().collect();
        let target_false = tokens[3].parse().unwrap();

        Monkey {
            items,
            op,
            test_divisor,
            target_true,
            target_false,
            inspected_items: 0,
        }
    }

    fn inspect(&mut self, worry_divisor: Option<usize>, modulo: usize) -> Vec<(usize, usize)> {
        let mut throw_items = Vec::with_capacity(self.items.len());

        while let Some(mut worry_level) = self.items.pop_front() {
            // Monkey inspects, worry level increases
            worry_level = match self.op {
                MonkeyOp::Add(n) => worry_level + n,
                MonkeyOp::AddSelf => worry_level + worry_level,
                MonkeyOp::Mul(n) => worry_level * n,
                MonkeyOp::MulSelf => worry_level * worry_level,
            };
            // Monkey gets bored
            if let Some(worry_divisor) = worry_divisor {
                worry_level /= worry_divisor;
            }
            // mod lcm of all monkey divisors, so values "wrap" and do
            // not grow indefinitely. Using lcm ensures all modulo
            // checks will still evaluate identical.
            worry_level %= modulo;

            // Test and throw
            if worry_level % self.test_divisor == 0 {
                throw_items.push((self.target_true, worry_level));
            } else {
                throw_items.push((self.target_false, worry_level));
            }

            // Update state
            self.inspected_items += 1;
        }

        throw_items
    }

    fn add_to_list(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn run(mut monkeys: Vec<Monkey>, rounds: usize, worry_divisor: Option<usize>) -> usize {
    // Get lcm for modulo wrapping of worry values. All seen divisors
    // in input are prime, so lcm is just the product.
    let divisors: HashSet<_> = monkeys.iter().map(|m| m.test_divisor).collect();
    let modulo = divisors.iter().product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let throw_items = monkeys[i].inspect(worry_divisor, modulo);
            for (throw_target, item) in throw_items.into_iter() {
                monkeys[throw_target].add_to_list(item);
            }
        }
    }

    let mut inspected_items: Vec<_> = monkeys.iter().map(|m| m.inspected_items).collect();
    inspected_items.sort_by(|a, b| b.cmp(a)); // descending sort
    inspected_items[0] * inspected_items[1]
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let monkeys: Vec<_> = input.split("\n\n").map(Monkey::parse).collect();
        format!("{}", run(monkeys, 20, Some(3)))
    }

    fn star2(&self, input: &str) -> String {
        let monkeys: Vec<_> = input.split("\n\n").map(Monkey::parse).collect();
        format!("{}", run(monkeys, 10000, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

        let d = Day11 {};
        assert_eq!(d.star1(input), "10605");
        assert_eq!(d.star2(input), "2713310158");
    }
}
