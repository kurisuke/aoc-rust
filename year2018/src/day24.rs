use common::day::Day;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Group {
    army: usize,
    units: usize,
    hp: usize,
    at_dmg: usize,
    at_type: String,
    weak: HashSet<String>,
    immune: HashSet<String>,
    ini: usize,
}

impl Group {
    fn alive(&self) -> bool {
        self.units > 0
    }

    fn effective_power(&self) -> usize {
        self.units * self.at_dmg
    }

    fn dmg_to(&self, target: &Group) -> usize {
        #[allow(clippy::bool_to_int_with_if)]
        let factor = if target.weak.contains(&self.at_type) {
            2
        } else if target.immune.contains(&self.at_type) {
            0
        } else {
            1
        };
        self.effective_power() * factor
    }

    fn take_dmg(&mut self, dmg: usize) {
        let killed_units = dmg / self.hp;
        if killed_units > self.units {
            self.units = 0;
        } else {
            self.units -= killed_units;
        }
    }
}

type Armies = HashMap<usize, Group>;

fn select_order(armies: &Armies) -> Vec<usize> {
    let mut v: Vec<_> = armies.values().filter(|g| g.alive()).collect();
    v.sort_by(|g1, g2| {
        g2.effective_power()
            .cmp(&g1.effective_power())
            .then_with(|| g2.ini.cmp(&g1.ini))
    });
    v.into_iter().map(|g| g.ini).collect()
}

fn select_phase(armies: &Armies) -> HashMap<usize, usize> {
    let mut targets = HashMap::new();
    let mut already_chosen = HashSet::new();
    for attacker in select_order(armies) {
        let group_attacker = armies.get(&attacker).unwrap();
        let candidate = armies
            .values()
            .filter(|g| {
                g.alive()
                    && !already_chosen.contains(&g.ini)
                    && g.army != group_attacker.army
                    && group_attacker.dmg_to(g) > 0
            })
            .max_by(|g1, g2| {
                group_attacker
                    .dmg_to(g1)
                    .cmp(&group_attacker.dmg_to(g2))
                    .then_with(|| g1.effective_power().cmp(&g2.effective_power()))
                    .then_with(|| g1.ini.cmp(&g2.ini))
            });
        if let Some(group_target) = candidate {
            targets.insert(attacker, group_target.ini);
            already_chosen.insert(group_target.ini);
        }
    }
    targets
}

fn attack_phase(armies: &mut Armies, targets: &HashMap<usize, usize>) {
    for attacker in (1..=20).rev() {
        if let Some(target) = targets.get(&attacker) {
            let mut dmg = 0;
            {
                let group_attacker = armies.get(&attacker).unwrap();
                let group_target = armies.get(target).unwrap();
                if group_attacker.alive() && group_target.alive() {
                    dmg = group_attacker.dmg_to(group_target)
                }
            }
            if dmg > 0 {
                let group_target = armies.get_mut(target).unwrap();
                group_target.take_dmg(dmg);
            }
        }
    }
}

fn combat(armies: &mut Armies) {
    let mut last_state = combat_result(armies);
    while armies.values().filter(|g| g.army == 0).any(|g| g.alive())
        && armies.values().filter(|g| g.army == 1).any(|g| g.alive())
    {
        let targets = select_phase(armies);
        attack_phase(armies, &targets);
        let new_state = combat_result(armies);
        if last_state == new_state {
            // stalemate
            break;
        } else {
            last_state = new_state;
        }
    }
}

fn combat_result(armies: &Armies) -> (usize, usize) {
    let units0 = armies
        .values()
        .filter(|g| g.army == 0)
        .map(|g| g.units)
        .sum::<usize>();
    let units1 = armies
        .values()
        .filter(|g| g.army == 1)
        .map(|g| g.units)
        .sum::<usize>();
    (units0, units1)
}

fn parse_army(input: &str, i: usize) -> Armies {
    input.lines().skip(1).map(|l| {
        let p: Vec<_> = l.split(&['(', ')'][..]).collect();
        match p.len() {
            1 => {
                let (units, hp, at_dmg, at_type, ini) =
                    scan_fmt!(p[0], "{d} units each with {d} hit points with an attack that does {d} {[a-z]} damage at initiative {d}",
                              usize, usize, usize, String, usize).unwrap();
                (ini, Group {
                    army: i,
                    units,
                    hp,
                    at_dmg,
                    at_type,
                    weak: HashSet::new(),
                    immune: HashSet::new(),
                    ini
                })
            }
            3 => {
                let (units, hp) =
                    scan_fmt!(p[0], "{d} units each with {d} hit points", usize, usize).unwrap();
                let (at_dmg, at_type, ini) =
                    scan_fmt!(p[2], "with an attack that does {d} {[a-z]} damage at initiative {d}", usize, String, usize).unwrap();

                let mut weak = HashSet::new();
                let mut immune = HashSet::new();
                for s in p[1].split("; ") {
                    let sw: Vec<_> = s.split(' ').collect();
                    match sw[0] {
                        "weak" => {
                            for w in sw[2..].iter() {
                                weak.insert(w.replace(',', ""));
                            }
                        }
                        "immune" => {
                            for w in sw[2..].iter() {
                                immune.insert(w.replace(',', ""));
                            }
                        }
                        _ => { unreachable!(); }
                    }
                }
                (ini, Group {
                    army: i,
                    units,
                    hp,
                    at_dmg,
                    at_type,
                    weak,
                    immune,
                    ini
                })
            }
            _ => unreachable!()
        }
    }).collect()
}

fn immune_boost(armies: &mut Armies, boost: usize) {
    for g in armies.values_mut() {
        if g.army == 0 {
            g.at_dmg += boost;
        }
    }
}

fn parse_input(input: &str) -> Armies {
    input
        .split("\n\n")
        .enumerate()
        .flat_map(|(i, a)| parse_army(a, i))
        .collect()
}

pub struct Day24 {}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let mut armies = parse_input(input);
        combat(&mut armies);
        let (units0, units1) = combat_result(&armies);
        format!("{}", units0.max(units1))
    }

    fn star2(&self, input: &str) -> String {
        let armies_orig = parse_input(input);
        let mut boost = 1;
        loop {
            let mut armies = armies_orig.clone();
            immune_boost(&mut armies, boost);
            combat(&mut armies);
            let (units0, units1) = combat_result(&armies);
            if units0 > 0 && units1 == 0 {
                return format!("{}", units0);
            } else {
                boost += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"#;

        let d = Day24 {};
        assert_eq!(d.star1(input), "5216");
        assert_eq!(d.star2(input), "51");
    }
}
