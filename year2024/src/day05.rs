use std::collections::HashMap;

use common::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let (page_orderings, updates) = parse_input(input);
        let sum = updates
            .iter()
            .filter(|update| page_orderings.iter().all(|ordering| ordering.is_ok(update)))
            .map(|update| update.middle)
            .sum::<usize>();
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let (page_orderings, updates) = parse_input(input);
        let mut sum = 0;

        for update in updates {
            let mut page_numbers_old = update.page_numbers.clone();
            let page_numbers_sorted = loop {
                let mut page_numbers_new = page_numbers_old.clone();
                for o in &page_orderings {
                    if let (Some(idx_before), Some(idx_after)) = (
                        page_numbers_new.iter().position(|n| *n == o.before),
                        page_numbers_new.iter().position(|n| *n == o.after),
                    ) {
                        if idx_after < idx_before {
                            page_numbers_new.swap(idx_after, idx_before);
                        }
                    }
                }
                if page_numbers_new == page_numbers_old {
                    break page_numbers_new;
                }
                page_numbers_old = page_numbers_new;
            };
            if page_numbers_sorted != update.page_numbers {
                sum += page_numbers_sorted[page_numbers_sorted.len() / 2];
            }
        }
        format!("{}", sum)
    }
}

struct PageOrdering {
    before: usize,
    after: usize,
}

impl PageOrdering {
    fn is_ok(&self, update: &Update) -> bool {
        if let (Some(before_pos), Some(after_pos)) = (
            update.positions.get(&self.before),
            update.positions.get(&self.after),
        ) {
            before_pos < after_pos
        } else {
            true
        }
    }
}

struct Update {
    pub page_numbers: Vec<usize>,
    pub positions: HashMap<usize, usize>,
    pub middle: usize,
}

fn parse_input(input: &str) -> (Vec<PageOrdering>, Vec<Update>) {
    let mut secs = input.split("\n\n");

    let page_orderings = secs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut spl = line.split('|');
            PageOrdering {
                before: spl.next().unwrap().parse().unwrap(),
                after: spl.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let updates = secs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let page_numbers: Vec<_> = line.split(',').map(|n| n.parse().unwrap()).collect();
            let middle = page_numbers[page_numbers.len() / 2];

            let positions = page_numbers
                .iter()
                .enumerate()
                .map(|(i, n)| (*n, i))
                .collect();

            Update {
                page_numbers,
                positions,
                middle,
            }
        })
        .collect();

    (page_orderings, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn star1() {
        let d = Day05 {};
        assert_eq!(d.star1(INPUT), "143");
    }

    #[test]
    fn star2() {
        let d = Day05 {};
        assert_eq!(d.star2(INPUT), "123");
    }
}
