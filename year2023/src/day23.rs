use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use common::day::Day;
use itertools::Itertools;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day23 {}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        search_longest_path(&grid, true).unwrap().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        search_longest_path(&grid, false).unwrap().to_string()
    }
}

fn search_longest_path(grid: &Grid2D<char>, icy_path: bool) -> Option<usize> {
    let start_pos = Coords { y: 0, x: 1 };
    let end_pos = Coords {
        y: grid.height() - 1,
        x: grid.width() - 2,
    };

    let mut crossings = find_crossings(grid);
    crossings.insert(start_pos);
    crossings.insert(end_pos);

    let mut distances = HashMap::new();

    if icy_path {
        for (pos1, pos2) in crossings.iter().cartesian_product(crossings.iter()) {
            if pos1 != pos2 {
                if let Some(steps) = find_path(grid, &crossings, *pos1, *pos2, icy_path) {
                    distances.insert((*pos1, *pos2), steps);
                }
            }
        }
    } else {
        for pos in crossings.iter().combinations(2) {
            let pos1 = pos[0];
            let pos2 = pos[1];
            if let Some(steps) = find_path(grid, &crossings, *pos1, *pos2, icy_path) {
                distances.insert((*pos1, *pos2), steps);
                distances.insert((*pos2, *pos1), steps);
            }
        }
    }

    let mut path = vec![start_pos];
    find_max_path(&distances, 0, &mut path, &end_pos)
}

fn find_crossings(grid: &Grid2D<char>) -> HashSet<Coords> {
    grid.coords_iter()
        .filter(|pos| {
            grid.at(pos).unwrap() == &'.'
                && grid
                    .neighbors_cardinal(pos)
                    .into_iter()
                    .filter(|c| c.is_some_and(|c| *c != '#'))
                    .count()
                    >= 3
        })
        .collect()
}

fn find_max_path(
    distances: &HashMap<(Coords, Coords), usize>,
    steps: usize,
    path: &mut Vec<Coords>,
    end: &Coords,
) -> Option<usize> {
    let pos = *path.last().unwrap();
    if &pos == end {
        return Some(steps);
    }

    let neighbors: Vec<_> = distances
        .keys()
        .filter(|(src, _)| src == &pos)
        .filter(|(_, dest)| !path.contains(dest))
        .to_owned()
        .collect();

    let mut max_steps = None;
    for neighbor in neighbors {
        path.push(neighbor.1);
        let new_steps = steps + distances.get(neighbor).unwrap();
        if let Some(new_steps) = find_max_path(distances, new_steps, path, end) {
            if let Some(old_steps) = max_steps {
                max_steps = Some(new_steps.max(old_steps));
            } else {
                max_steps = Some(new_steps);
            }
        }
        path.pop().unwrap();
    }

    max_steps
}

#[derive(PartialEq, Eq)]
struct SearchState {
    pos: Coords,
    steps: usize,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| (self.pos.x + self.pos.y).cmp(&(other.pos.x + other.pos.y)))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(
    grid: &Grid2D<char>,
    crossings: &HashSet<Coords>,
    start: Coords,
    end: Coords,
    icy_slopes: bool,
) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    visited.insert(start, 0);
    queue.push(SearchState {
        pos: start,
        steps: 0,
    });

    while let Some(state) = queue.pop() {
        if state.pos == end {
            return Some(state.steps);
        }

        if state.pos != start && crossings.contains(&state.pos) {
            // found another crossing, no need to continue here
            continue;
        }

        let next_poses = if icy_slopes {
            match grid.at(&state.pos).unwrap() {
                '.' => grid.neighbors_cardinal_coords(&state.pos),
                '^' => vec![state.pos.mov(Direction::N)],
                'v' => vec![state.pos.mov(Direction::S)],
                '<' => vec![state.pos.mov(Direction::W)],
                '>' => vec![state.pos.mov(Direction::E)],
                _ => unreachable!(),
            }
        } else {
            grid.neighbors_cardinal_coords(&state.pos)
        };

        let next_poses: Vec<_> = next_poses
            .into_iter()
            .filter(|p| grid.at(p).is_some_and(|c| *c != '#'))
            .collect();

        for next_pos in next_poses {
            let next_steps = state.steps + 1;
            if !visited.contains_key(&next_pos) || &next_steps < visited.get(&next_pos).unwrap() {
                visited.insert(next_pos, next_steps);
                queue.push(SearchState {
                    pos: next_pos,
                    steps: next_steps,
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn ex1() {
        let d = Day23 {};
        assert_eq!(d.star1(INPUT), "94");
    }

    #[test]
    fn ex2() {
        let d = Day23 {};
        assert_eq!(d.star2(INPUT), "154");
    }
}
