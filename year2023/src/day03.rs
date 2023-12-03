use std::collections::{HashMap, HashSet};

use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        part_numbers_and_gears(grid)
            .0
            .into_iter()
            .sum::<u32>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let gears = part_numbers_and_gears(grid).1;

        gears
            .values()
            .map(|part_numbers| {
                if part_numbers.len() > 1 {
                    part_numbers.iter().product::<u32>()
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }
}

#[derive(PartialEq, Eq)]
enum NumberParseState {
    None,
    PartNumber,
    NonPartNumber,
}

fn part_numbers_and_gears(grid: Grid2D<char>) -> (Vec<u32>, HashMap<Coords, Vec<u32>>) {
    let mut part_numbers = vec![];
    let mut gears = HashMap::new();

    for y in 0..grid.height() {
        let mut state = NumberParseState::None;
        let mut part_number = 0;
        let mut gear_neighbor_set = HashSet::new();

        for x in 0..grid.width() {
            let coords = Coords { x, y };
            let val = grid.at(&coords).unwrap();

            match state {
                NumberParseState::None => {
                    if val.is_ascii_digit() {
                        part_number += val.to_digit(10).unwrap();
                        gear_neighbor_set.extend(gear_neighbors(&grid, &coords).into_iter());

                        if has_symbol_neighbor(&grid, &coords) {
                            state = NumberParseState::PartNumber;
                        } else {
                            state = NumberParseState::NonPartNumber;
                        }
                    }
                }
                NumberParseState::PartNumber => {
                    if val.is_ascii_digit() {
                        part_number *= 10;
                        part_number += val.to_digit(10).unwrap();
                        gear_neighbor_set.extend(gear_neighbors(&grid, &coords).into_iter());
                    } else {
                        part_numbers.push(part_number);
                        for gear in &gear_neighbor_set {
                            let e = gears.entry(*gear).or_insert(vec![]);
                            e.push(part_number);
                        }
                        gear_neighbor_set.clear();
                        part_number = 0;
                        state = NumberParseState::None;
                    }
                }
                NumberParseState::NonPartNumber => {
                    if val.is_ascii_digit() {
                        part_number *= 10;
                        part_number += val.to_digit(10).unwrap();
                        gear_neighbor_set.extend(gear_neighbors(&grid, &coords).into_iter());
                        if has_symbol_neighbor(&grid, &coords) {
                            state = NumberParseState::PartNumber;
                        }
                    } else {
                        gear_neighbor_set.clear();
                        part_number = 0;
                        state = NumberParseState::None;
                    }
                }
            }
        }

        // row end
        if state == NumberParseState::PartNumber {
            part_numbers.push(part_number);
            for gear in &gear_neighbor_set {
                let e = gears.entry(*gear).or_insert(vec![]);
                e.push(part_number);
            }
        }
    }
    (part_numbers, gears)
}

fn has_symbol_neighbor(grid: &Grid2D<char>, coords: &Coords) -> bool {
    grid.neighbors(coords)
        .into_iter()
        .any(|x| x.is_some_and(|x| !x.is_ascii_digit() && x != &'.'))
}

fn gear_neighbors(grid: &Grid2D<char>, coords: &Coords) -> Vec<Coords> {
    let gear_neighbors = grid
        .neighbors_coords(coords)
        .into_iter()
        .filter(|c| grid.at(c).is_some_and(|c| c == &'*'))
        .collect();
    gear_neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn ex1() {
        let d = Day03 {};
        assert_eq!(d.star1(INPUT), "4361");
    }

    #[test]
    fn ex2() {
        let d = Day03 {};
        assert_eq!(d.star2(INPUT), "467835");
    }
}
