use common::day::Day;
use util::grid2d::{Coords, Grid2D};

use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Coords,
    steps: usize,
}

struct Map {
    start: Coords,
    end: Coords,
    heightmap: Grid2D<isize>,
}

fn parse_input(input: &str) -> Map {
    let grid = Grid2D::new(input).unwrap();
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let heightmap = Grid2D::new_by(input, |c| match c {
        'a'..='z' => c as isize - 0x60,
        'S' => 1,
        'E' => 26,
        _ => unreachable!(),
    })
    .unwrap();

    Map {
        start,
        end,
        heightmap,
    }
}

fn search(map: Map) -> Option<usize> {
    let mut search_states = VecDeque::new();
    search_states.push_back(State {
        pos: map.start,
        steps: 0,
    });
    let mut visited = HashSet::new();
    visited.insert(map.start);

    while let Some(state) = search_states.pop_front() {
        if state.pos == map.end {
            return Some(state.steps);
        }

        let elevation_here = map.heightmap.at(&state.pos).unwrap();
        for npos in map
            .heightmap
            .neighbors_cardinal_coords(&state.pos)
            .into_iter()
        {
            if let Some(elevation_there) = map.heightmap.at(&npos) {
                if elevation_there - elevation_here <= 1 && !visited.contains(&npos) {
                    search_states.push_back(State {
                        pos: npos,
                        steps: state.steps + 1,
                    });
                    visited.insert(npos);
                }
            }
        }
    }

    None
}

fn search_pt2(map: Map) -> Option<usize> {
    let mut search_states = VecDeque::new();
    search_states.push_back(State {
        pos: map.end,
        steps: 0,
    });
    let mut visited = HashSet::new();
    visited.insert(map.end);

    while let Some(state) = search_states.pop_front() {
        if map.heightmap.at(&state.pos).unwrap() == &1 {
            return Some(state.steps);
        }

        let elevation_here = map.heightmap.at(&state.pos).unwrap();
        for npos in map
            .heightmap
            .neighbors_cardinal_coords(&state.pos)
            .into_iter()
        {
            if let Some(elevation_there) = map.heightmap.at(&npos) {
                if elevation_here - elevation_there <= 1 && !visited.contains(&npos) {
                    search_states.push_back(State {
                        pos: npos,
                        steps: state.steps + 1,
                    });
                    visited.insert(npos);
                }
            }
        }
    }

    None
}

pub struct Day12 {}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let map = parse_input(input);
        format!("{}", search(map).unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let map = parse_input(input);
        format!("{}", search_pt2(map).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        let d = Day12 {};
        assert_eq!(d.star1(input), "31");
        assert_eq!(d.star2(input), "29");
    }
}
