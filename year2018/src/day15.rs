use common::day::Day;
use std::collections::{BTreeSet, HashSet, VecDeque};
use util::grid2d::{Coords, Grid2D};

pub struct Day15 {}

#[derive(Copy, Clone)]
enum Field {
    Wall,
    Open,
    Goblin(Unit),
    Elf(Unit),
}

#[derive(Copy, Clone)]
enum TargetType {
    Goblin,
    Elf,
}

#[derive(Copy, Clone)]
struct Unit {
    pub power: u32,
    pub hp: u32,
}

fn find_targets(grid: &Grid2D<Field>, target_type: TargetType) -> BTreeSet<Coords> {
    let mut target_coords = BTreeSet::new();
    for (coords, field) in grid.enumerate() {
        match target_type {
            TargetType::Goblin => {
                if let Field::Goblin(_) = field {
                    target_coords.insert(coords);
                }
            }
            TargetType::Elf => {
                if let Field::Elf(_) = field {
                    target_coords.insert(coords);
                }
            }
        }
    }
    target_coords
}

fn find_adjacent(grid: &Grid2D<Field>, targets: &BTreeSet<Coords>) -> BTreeSet<Coords> {
    let mut adjacents = BTreeSet::new();
    for target in targets {
        for neighbor in grid.neighbors_cardinal_coords(target) {
            if let Some(Field::Open) = grid.at(&neighbor) {
                    adjacents.insert(neighbor);
            }
        }
    }
    adjacents
}

fn bfs(grid: &Grid2D<Field>, src: &Coords, dests: &BTreeSet<Coords>) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((*src, 0));
    visited.insert(*src);

    while let Some((f, dist)) = frontier.pop_front() {
        if dests.contains(&f) {
            return Some(dist);
        } else {
            for n in grid.neighbors_cardinal_coords(&f) {
                if let Some(Field::Open) = grid.at(&n) {
                    if !visited.contains(&n) {
                        frontier.push_back((n, dist + 1));
                        visited.insert(n);
                    }
                }
            }
        }
    }
    None
}

fn find_move_step(
    grid: &Grid2D<Field>,
    unit_pos: &Coords,
    adjacents: &BTreeSet<Coords>,
) -> Option<Coords> {
    // for all neighboring fields
    let move_cands = grid.neighbors_cardinal_coords(unit_pos);
    let mut move_cands: Vec<(usize, Coords)> = move_cands
        .into_iter()
        .filter_map(|cand| {
            if let Some(Field::Open) = grid.at(&cand) {
                if let Some(dist) = bfs(grid, &cand, adjacents) {
                    return Some((dist, cand));
                }
            }
            None
        })
        .collect();
    // chose neighboring field with shortest path. on tie, choose first in order
    move_cands.sort();

    move_cands.into_iter().map(|(_, pos)| pos).next()
}

fn find_in_range(
    grid: &Grid2D<Field>,
    unit_pos: &Coords,
    target_type: TargetType,
) -> Option<Coords> {
    // check if neighboring fields have an enemy unit
    let mut targets = vec![];
    for n in grid.neighbors_cardinal_coords(unit_pos) {
        if let Some(field) = grid.at(&n) {
            match target_type {
                TargetType::Elf => {
                    if let Field::Elf(unit) = field {
                        targets.push((unit.hp, n));
                    }
                }
                TargetType::Goblin => {
                    if let Field::Goblin(unit) = field {
                        targets.push((unit.hp, n));
                    }
                }
            }
        }
    }
    targets.sort();

    // if there are several, take the one with the least HP
    targets.into_iter().map(|(_, pos)| pos).next()
}

fn run(grid: &mut Grid2D<Field>) -> usize {
    let mut round_counter = 0;
    loop {
        if round(grid) {
            break;
        }
        round_counter += 1;
    }
    let hp_left: usize = grid
        .iter()
        .map(|f| match f {
            Field::Goblin(unit) | Field::Elf(unit) => unit.hp as usize,
            _ => 0,
        })
        .sum();
    round_counter * hp_left
}

fn round(grid: &mut Grid2D<Field>) -> bool {
    // get positions of all units at start of round
    let all_unit_pos: Vec<Coords> = grid
        .enumerate()
        .filter(|(_, f)| matches!(f, Field::Elf(_) | Field::Goblin(_)))
        .map(|(c, _)| c)
        .collect();

    for unit_pos in all_unit_pos.iter() {
        let combat_ended = turn(grid, unit_pos);
        if combat_ended {
            return true;
        }
    }
    false
}

fn turn(grid: &mut Grid2D<Field>, unit_pos: &Coords) -> bool {
    // check if unit on field still alive
    let (target_type, power) = match grid.at(unit_pos).unwrap() {
        Field::Elf(x) => (TargetType::Goblin, x.power),
        Field::Goblin(x) => (TargetType::Elf, x.power),
        _ => {
            return false;
        }
    };

    // check if in attack range == adjacent to an enemy unit
    if let Some(attack_target) = find_in_range(grid, &unit_pos, target_type) {
        do_attack(grid, &attack_target, power);
    } else {
        // try to move
        let targets = find_targets(grid, target_type);
        if targets.is_empty() {
            // no more enemies, combat ended
            return true;
        }

        // check if there are free fields next to a target
        let adjacents = find_adjacent(grid, &targets);
        if adjacents.is_empty() {
            // no place where we can move, so unit ends turn
            return false;
        }

        if let Some(new_unit_pos) = find_move_step(grid, unit_pos, &adjacents) {
            do_move(grid, unit_pos, &new_unit_pos);
            let unit_pos = new_unit_pos;

            // after we moved, check if we can attack now
            if let Some(attack_target) = find_in_range(grid, &unit_pos, target_type) {
                do_attack(grid, &attack_target, power);
            }
        }
    }

    false
}

fn do_attack(grid: &mut Grid2D<Field>, attack_target: &Coords, power: u32) {
    match grid.at(attack_target).unwrap() {
        Field::Elf(unit) => {
            if unit.hp <= power {
                // unit is killed
                grid.set(attack_target, Field::Open);
            } else {
                let mut unit = *unit;
                unit.hp -= power;
                grid.set(attack_target, Field::Elf(unit));
            }
        }
        Field::Goblin(unit) => {
            if unit.hp <= power {
                // unit is killed
                grid.set(attack_target, Field::Open);
            } else {
                let mut unit = *unit;
                unit.hp -= power;
                grid.set(attack_target, Field::Goblin(unit));
            }
        }
        _ => {
            panic!("attack: target is not a unit");
        }
    }
}

fn do_move(grid: &mut Grid2D<Field>, unit_pos: &Coords, new_unit_pos: &Coords) {
    let unit = *grid.at(unit_pos).unwrap();
    grid.set(new_unit_pos, unit);
    grid.set(unit_pos, Field::Open);
}

fn parse_input(input: &str) -> Grid2D<Field> {
    Grid2D::new_by(input, |c| match c {
        'G' => Field::Goblin(Unit { power: 3, hp: 200 }),
        'E' => Field::Elf(Unit { power: 3, hp: 200 }),
        '.' => Field::Open,
        '#' => Field::Wall,
        _ => {
            panic!("Invalid char: {}", c);
        }
    })
    .unwrap()
}

#[allow(dead_code)]
fn grid_to_str(grid: &Grid2D<Field>) -> String {
    let mut s = String::new();
    for (pos, field) in grid.enumerate() {
        if pos.y > 0 && pos.x == 0 {
            s.push('\n');
        }
        s.push(match field {
            Field::Wall => '#',
            Field::Open => '.',
            Field::Goblin(_) => 'G',
            Field::Elf(_) => 'E',
        });
    }
    s
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let mut grid = parse_input(input);
        format!("{}", run(&mut grid))
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day15 {};

        let input1 = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#;
        assert_eq!(d.star1(input1), "27730");

        let input2 = r#"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"#;
        assert_eq!(d.star1(input2), "36334");

        let input3 = r#"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#;
        assert_eq!(d.star1(input3), "39514");

        let input4 = r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#;
        assert_eq!(d.star1(input4), "27755");

        let input5 = r#"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#;
        assert_eq!(d.star1(input5), "28944");

        let input6 = r#"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"#;
        assert_eq!(d.star1(input6), "18740");
    }
}
