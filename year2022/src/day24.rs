use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use common::day::Day;
use util::{
    gcd::lcm,
    grid2d::{Coords, Direction, Grid2D},
};

pub struct Day24 {}

struct Map {
    grid_bas: Grid2D<char>,
    blizzards: Vec<Blizzard>,
}

struct Blizzard {
    init_pos: Coords,
    direction: Direction,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct SearchState {
    minute: usize,
    pos: Coords,
    distance: usize,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.minute + other.distance)
            .cmp(&(self.minute + self.distance))
            .then_with(|| other.pos.cmp(&self.pos))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Map {
    let mut grid_bas = Grid2D::new(input).unwrap();
    let mut blizzards = vec![];
    for (ch, direction) in [
        ('^', Direction::N),
        ('v', Direction::S),
        ('<', Direction::W),
        ('>', Direction::E),
    ] {
        for init_pos in grid_bas.filter(&[ch]) {
            blizzards.push(Blizzard {
                init_pos,
                direction,
            })
        }
    }

    grid_bas.replace(&'^', &'.');
    grid_bas.replace(&'v', &'.');
    grid_bas.replace(&'<', &'.');
    grid_bas.replace(&'>', &'.');

    Map {
        grid_bas,
        blizzards,
    }
}

fn gen_grids(map: &Map) -> Vec<Grid2D<char>> {
    let cycle_length = Coords {
        x: map.grid_bas.width() - 2,
        y: map.grid_bas.height() - 2,
    };
    let cycle_length_total = lcm(cycle_length.x as usize, cycle_length.y as usize);

    let mut grids = vec![];
    for offset in 0..cycle_length_total {
        let mut grid = map.grid_bas.clone();
        for b in map.blizzards.iter() {
            let mut pos = b.init_pos;
            pos.x -= 1;
            pos.y -= 1;

            match b.direction {
                Direction::N => {
                    pos.y = (pos.y - offset as i64).rem_euclid(cycle_length.y);
                }
                Direction::S => {
                    pos.y = (pos.y + offset as i64).rem_euclid(cycle_length.y);
                }
                Direction::W => {
                    pos.x = (pos.x - offset as i64).rem_euclid(cycle_length.x);
                }
                Direction::E => {
                    pos.x = (pos.x + offset as i64).rem_euclid(cycle_length.x);
                }
                _ => unreachable!(),
            }

            pos.x += 1;
            pos.y += 1;

            assert!(grid.set(&pos, '#'));
        }
        grids.push(grid);
    }

    grids
}

fn get_start_end(grids: &[Grid2D<char>]) -> (Coords, Coords) {
    let start = Coords { x: 1, y: 0 };
    let end = Coords {
        x: grids[0].width() - 2,
        y: grids[0].height() - 1,
    };
    (start, end)
}

fn search(grids: &[Grid2D<char>], start: Coords, end: Coords, time_offset: usize) -> Option<usize> {
    let init_state = SearchState {
        minute: time_offset,
        pos: start,
        distance: start.manhattan(&end) as usize,
    };
    let mut visited = HashSet::new();
    visited.insert(init_state);
    let mut queue = BinaryHeap::new();
    queue.push(init_state);

    while let Some(state) = queue.pop() {
        if state.pos == end {
            return Some(state.minute);
        }

        let grid_next = &grids[(state.minute + 1) % grids.len()];
        // stay in same pos, or move in 4 cardinal directions
        for new_pos in [
            state.pos,
            state.pos.mov(Direction::N),
            state.pos.mov(Direction::S),
            state.pos.mov(Direction::W),
            state.pos.mov(Direction::E),
        ] {
            if let Some('.') = grid_next.at(&new_pos) {
                let next_state = SearchState {
                    minute: state.minute + 1,
                    pos: new_pos,
                    distance: new_pos.manhattan(&end) as usize,
                };
                if visited.insert(next_state) {
                    queue.push(next_state);
                }
            }
        }
    }
    None
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let map = parse_input(input);
        let grids = gen_grids(&map);
        let (start, end) = get_start_end(&grids);
        format!("{}", search(&grids, start, end, 0).unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let map = parse_input(input);
        let grids = gen_grids(&map);
        let (start, end) = get_start_end(&grids);
        let offset1 = search(&grids, start, end, 0).unwrap();
        let offset2 = search(&grids, end, start, offset1).unwrap();
        format!("{}", search(&grids, start, end, offset2).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(100, 35), 700);

        let d = Day24 {};
        assert_eq!(d.star1(input), "18");
        assert_eq!(d.star2(input), "54");
    }
}
