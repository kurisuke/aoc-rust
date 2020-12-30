use crate::day::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};

pub struct Day11 {}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
enum Typ {
    Gen,
    Chip,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct Equipment<'a> {
    typ: Typ,
    elem: &'a str,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State<'a> {
    equipment: BTreeMap<Equipment<'a>, usize>,
    elevator: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct EquivState {
    pairs: Vec<(usize, usize)>,
    elevator: usize,
}

fn to_equiv(state: &State) -> EquivState {
    let elems: HashSet<_> = state.equipment.keys().map(|e| e.elem).collect();
    let mut pairs: Vec<_> = elems
        .iter()
        .map(|elem| {
            (
                *state
                    .equipment
                    .get(&Equipment {
                        typ: Typ::Gen,
                        elem,
                    })
                    .unwrap(),
                *state
                    .equipment
                    .get(&Equipment {
                        typ: Typ::Chip,
                        elem,
                    })
                    .unwrap(),
            )
        })
        .collect();
    pairs.sort_unstable();
    EquivState {
        pairs,
        elevator: state.elevator,
    }
}

fn is_valid(state: &State) -> bool {
    // check if any chips are being fried
    for (chip, chip_floor) in state.equipment.iter().filter(|(e, _)| e.typ == Typ::Chip) {
        // is own generator on same floor?
        let own_gen_floor = state
            .equipment
            .get(&Equipment {
                typ: Typ::Gen,
                elem: chip.elem,
            })
            .unwrap();
        if chip_floor != own_gen_floor {
            // own generator not on same floor --> chip is unpowered
            // in this case: if there are other generators on the same floor, the chip is fried --> invalid state
            if state
                .equipment
                .iter()
                .filter(|(gen, _)| gen.typ == Typ::Gen && gen.elem != chip.elem)
                .any(|(_, gen_floor)| gen_floor == chip_floor)
            {
                return false;
            }
        }
    }
    true
}

fn next_states<'a>(
    border: &HashSet<State<'a>>,
    inside: &HashSet<EquivState>,
) -> HashSet<State<'a>> {
    let mut next_border = HashSet::new();
    for state in border {
        // possible move directions for elevator
        let mut target_floors = vec![];
        if state.elevator < 4 {
            // move up if not on top floor
            target_floors.push(state.elevator + 1);
        }
        if state.elevator > 1 {
            // move down if not on bottom floor
            target_floors.push(state.elevator - 1);
        }
        for target_floor in target_floors {
            let cur_floor_equipment = state
                .equipment
                .iter()
                .filter(|(_, f)| **f == state.elevator)
                .map(|(k, _)| k);
            for take_two in cur_floor_equipment.combinations(2) {
                let mut new_state = state.clone();
                new_state
                    .equipment
                    .entry(take_two[0].clone())
                    .and_modify(|f| *f = target_floor);
                new_state
                    .equipment
                    .entry(take_two[1].clone())
                    .and_modify(|f| *f = target_floor);
                new_state.elevator = target_floor;
                if !next_border.contains(&new_state)
                    && !border.contains(&new_state)
                    && !inside.contains(&to_equiv(&new_state))
                    && is_valid(&new_state)
                {
                    next_border.insert(new_state);
                }
            }

            let cur_floor_equipment = state
                .equipment
                .iter()
                .filter(|(_, f)| **f == state.elevator)
                .map(|(k, _)| k);
            for take_one in cur_floor_equipment {
                let mut new_state = state.clone();
                new_state
                    .equipment
                    .entry(take_one.clone())
                    .and_modify(|f| *f = target_floor);
                new_state.elevator = target_floor;
                if !next_border.contains(&new_state)
                    && !border.contains(&new_state)
                    && !inside.contains(&to_equiv(&new_state))
                    && is_valid(&new_state)
                {
                    next_border.insert(new_state);
                }
            }
        }
    }
    next_border
}

fn search(init_state: State, target_state: State) -> Option<usize> {
    let mut inside = HashSet::new();
    let mut border = HashSet::new();
    border.insert(init_state);
    let mut steps = 0;
    while !border.is_empty() && !border.contains(&target_state) {
        let next_border = next_states(&border, &inside);
        let border_equiv: HashSet<_> = border.iter().map(|x| to_equiv(x)).collect();
        inside.extend(border_equiv);
        border = next_border;
        steps += 1
    }
    if border.is_empty() {
        None
    } else {
        Some(steps)
    }
}

fn parse_input(input: &str) -> (State, State) {
    let mut init_equipment = BTreeMap::new();
    let re_gen = Regex::new(r"(\w+)\s+generator").unwrap();
    let re_chip = Regex::new(r"(\w+)-compatible\s+microchip").unwrap();
    for (floor, line) in input.lines().enumerate() {
        for caps in re_gen.captures_iter(line) {
            init_equipment.insert(
                Equipment {
                    typ: Typ::Gen,
                    elem: caps.get(1).unwrap().as_str(),
                },
                floor + 1,
            );
        }
        for caps in re_chip.captures_iter(line) {
            init_equipment.insert(
                Equipment {
                    typ: Typ::Chip,
                    elem: caps.get(1).unwrap().as_str(),
                },
                floor + 1,
            );
        }
    }
    let mut target_equipment = init_equipment.clone();
    for v in target_equipment.values_mut() {
        *v = 4;
    }
    (
        State {
            equipment: init_equipment,
            elevator: 1,
        },
        State {
            equipment: target_equipment,
            elevator: 4,
        },
    )
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let (init_state, target_state) = parse_input(input);
        format!("{}", search(init_state, target_state).unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let (mut init_state, mut target_state) = parse_input(input);

        init_state.equipment.insert(
            Equipment {
                typ: Typ::Gen,
                elem: "elerium",
            },
            1,
        );
        init_state.equipment.insert(
            Equipment {
                typ: Typ::Chip,
                elem: "elerium",
            },
            1,
        );
        init_state.equipment.insert(
            Equipment {
                typ: Typ::Gen,
                elem: "dilithium",
            },
            1,
        );
        init_state.equipment.insert(
            Equipment {
                typ: Typ::Chip,
                elem: "dilithium",
            },
            1,
        );

        target_state.equipment.insert(
            Equipment {
                typ: Typ::Gen,
                elem: "elerium",
            },
            4,
        );
        target_state.equipment.insert(
            Equipment {
                typ: Typ::Chip,
                elem: "elerium",
            },
            4,
        );
        target_state.equipment.insert(
            Equipment {
                typ: Typ::Gen,
                elem: "dilithium",
            },
            4,
        );
        target_state.equipment.insert(
            Equipment {
                typ: Typ::Chip,
                elem: "dilithium",
            },
            4,
        );

        format!("{}", search(init_state, target_state).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day11 {};
        let input = r#"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant."#;
        assert_eq!(d.star1(input), "11");
    }
}
