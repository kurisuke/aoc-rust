use common::day::Day;
use std::collections::{BTreeSet, HashSet, VecDeque};
use util::grid2d::{Coords, Grid2D};

pub struct Day24 {}

#[derive(Clone, Eq, PartialEq, Hash)]
struct BfsState {
    pos: Coords,
    visited: BTreeSet<char>,
    steps: usize,
}

fn neighbors(grid: &Grid2D<char>, pos: &Coords) -> Vec<Coords> {
    grid.neighbors_cardinal_coords(pos)
        .into_iter()
        .filter(|c| {
            let v = grid.at(c);
            v.is_some() && v.unwrap() != &'#'
        })
        .collect()
}

fn search(grid: &Grid2D<char>, from: &Coords, max_num: usize, no_return: bool) -> Option<usize> {
    let mut frontier = VecDeque::new();
    let mut reached = HashSet::new();
    let start = BfsState {
        pos: *from,
        visited: vec!['0'].into_iter().collect(),
        steps: 0,
    };
    frontier.push_back(start.clone());
    reached.insert((start.pos, start.visited));

    while !frontier.is_empty() {
        let mut current = frontier.pop_front().unwrap();
        let ch = grid.at(&current.pos).unwrap();
        if ch.is_ascii_digit() {
            current.visited.insert(*ch);
            if (current.visited.len() == max_num + 1) && (no_return || *ch == '0') {
                return Some(current.steps);
            }
        }

        for next in neighbors(grid, &current.pos) {
            if reached.insert((next, current.visited.clone())) {
                frontier.push_back(BfsState {
                    pos: next,
                    visited: current.visited.clone(),
                    steps: current.steps + 1,
                });
            }
        }
    }
    None
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let start_pos = grid.find('0').unwrap();
        let max_num: usize = ('1'..='9')
            .filter(|n| grid.find(*n).is_some())
            .max()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .unwrap();
        format!("{}", search(&grid, &start_pos, max_num, true).unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let start_pos = grid.find('0').unwrap();
        let max_num: usize = ('1'..='9')
            .filter(|n| grid.find(*n).is_some())
            .max()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .unwrap();
        format!("{}", search(&grid, &start_pos, max_num, false).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day24 {};
        let input = r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#;
        assert_eq!(d.star1(input), "14");
    }
}
