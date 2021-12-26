use common::day::Day;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day14 {}

struct Reactant<'a> {
    name: &'a str,
    qty: usize,
}

struct Reaction<'a> {
    inputs: Vec<Reactant<'a>>,
    product_qty: usize,
}

type Reactions<'a> = HashMap<&'a str, Reaction<'a>>;
type Stockpile<'a> = HashMap<String, usize>;

fn parse_input(input: &str) -> Reactions {
    input
        .lines()
        .map(|l| {
            let (input_str, output_str) = l.split_once(" => ").unwrap();
            let inputs = input_str
                .split(", ")
                .map(|r| {
                    let (qty_s, name) = r.split_once(' ').unwrap();
                    Reactant {
                        name,
                        qty: qty_s.parse().unwrap(),
                    }
                })
                .collect();

            let (product_qty_s, product_name) = output_str.split_once(' ').unwrap();
            (
                product_name,
                Reaction {
                    inputs,
                    product_qty: product_qty_s.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn produce(reactions: &Reactions, stockpile: &mut Stockpile, required: &Reactant) -> usize {
    // first special case: ore
    if required.name == "ORE" {
        return required.qty;
    }

    // first use the stuff we have on stock
    let reuse = required
        .qty
        .min(*stockpile.get(required.name).unwrap_or(&0));
    let net_qty = required.qty - reuse;
    *stockpile.entry(required.name.to_string()).or_insert(0) -= reuse;

    // if not, we have to produce a certain net quantity (that we cannot take from the stockpile)
    let reaction = reactions.get(required.name).unwrap();
    let eq_factor = if net_qty % reaction.product_qty > 0 {
        net_qty / reaction.product_qty + 1
    } else {
        net_qty / reaction.product_qty
    };

    // now we need to produce our requirements
    let mut ore = 0;
    for input in reaction.inputs.iter() {
        let input_req = eq_factor * input.qty;
        ore += produce(
            reactions,
            stockpile,
            &Reactant {
                name: input.name,
                qty: input_req,
            },
        );
    }

    // put excess on the stockpile
    let excess = eq_factor * reaction.product_qty - net_qty;
    *stockpile.entry(required.name.to_string()).or_insert(0) += excess;

    ore
}

fn binary_search(reactions: &Reactions, target_ore: usize) -> usize {
    let mut fuel_min = 0;
    let mut fuel_max = 2_usize.pow(31);

    while fuel_max - fuel_min > 1 {
        let pivot = (fuel_max + fuel_min) / 2;
        let mut stockpile = HashMap::new();
        let ore = produce(
            reactions,
            &mut stockpile,
            &Reactant {
                name: "FUEL",
                qty: pivot,
            },
        );
        match ore.cmp(&target_ore) {
            Ordering::Less => {
                fuel_min = pivot;
            }
            Ordering::Greater => {
                fuel_max = pivot;
            }
            Ordering::Equal => {
                fuel_min = pivot;
                fuel_max = pivot;
            }
        }
    }

    fuel_min
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let reactions = parse_input(input);
        let mut stockpile = HashMap::new();
        let ore = produce(
            &reactions,
            &mut stockpile,
            &Reactant {
                name: "FUEL",
                qty: 1,
            },
        );
        format!("{}", ore)
    }

    fn star2(&self, input: &str) -> String {
        let reactions = parse_input(input);
        format!("{}", binary_search(&reactions, 1000000000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#;

        let d = Day14 {};
        assert_eq!(d.star1(input), "165");
    }

    #[test]
    fn ex2() {
        let input = r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

        let d = Day14 {};
        assert_eq!(d.star1(input), "13312");
        assert_eq!(d.star2(input), "82892753");
    }

    #[test]
    fn ex3() {
        let input = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#;

        let d = Day14 {};
        assert_eq!(d.star1(input), "180697");
        assert_eq!(d.star2(input), "5586022");
    }

    #[test]
    fn ex4() {
        let input = r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#;

        let d = Day14 {};
        assert_eq!(d.star1(input), "2210736");
        assert_eq!(d.star2(input), "460664");
    }
}
