use crate::day::Day;
use std::collections::HashMap;

type BagRef = HashMap<String, u64>;
type BagRules = HashMap<String, BagRef>;

pub struct Day07 {}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let bag_rules = parse_input(input);
        let num_valid = bag_rules
            .keys()
            .filter(|color| {
                *color != "shiny gold" && has_child_bag(&bag_rules, color, "shiny gold")
            })
            .count();
        format!("{}", num_valid)
    }

    fn star2(&self, input: &str) -> String {
        let bag_rules = parse_input(input);
        let contained_bags = get_contained(&bag_rules, "shiny gold");
        let num_bags = contained_bags.values().sum::<u64>() - 1;
        format!("{}", num_bags)
    }
}

fn parse_input(input: &str) -> BagRules {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let mut lr_split = line.split(" bags contain ");
        let parent = lr_split.next().unwrap();
        let children = lr_split.next().unwrap();
        let mut child_refs: BagRef = HashMap::new();
        for child_str in children.split(", ") {
            if child_str.chars().next().unwrap().is_numeric() {
                let mut child_split = child_str.split_whitespace();
                let weight = child_split.next().unwrap().parse::<u64>().unwrap();
                let color = format!(
                    "{} {}",
                    child_split.next().unwrap(),
                    child_split.next().unwrap()
                );
                child_refs.insert(color, weight);
            }
        }
        rules.insert(String::from(parent), child_refs);
    }
    rules
}

fn has_child_bag(rules: &BagRules, parent_color: &str, search_color: &str) -> bool {
    if parent_color == search_color {
        true
    } else {
        let child_rules = &rules[parent_color];
        for child_color in child_rules.keys() {
            if has_child_bag(rules, child_color, search_color) {
                return true;
            }
        }
        false
    }
}

fn get_contained(rules: &BagRules, color: &str) -> BagRef {
    let child_rules = &rules[color];
    let mut all_contained = HashMap::new();

    // add self
    all_contained.insert(String::from(color), 1);

    // iterate over all bags which must be in this one
    for child_color in child_rules.keys() {
        // get all required bags recursively for 1 bag of the child color
        let child_contained = get_contained(rules, child_color);

        // how many bags of this color?
        let weight = child_rules[child_color];

        // add with the required weight
        for c in child_contained.keys() {
            let e = all_contained.entry(c.clone()).or_insert(0);
            *e += weight * child_contained[c];
        }
    }
    all_contained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day07 {};
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        assert_eq!(d.star1(input), "4");
        assert_eq!(d.star2(input), "32");
    }

    #[test]
    fn ex2() {
        let d = Day07 {};
        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(d.star2(input), "126");
    }
}
