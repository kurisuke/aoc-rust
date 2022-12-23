use std::collections::{HashMap, HashSet};

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day23 {}

fn parse_input(input: &str) -> HashSet<Coords> {
    let grid = Grid2D::new(input).unwrap();
    let elves = grid.filter(&['#']);
    elves.into_iter().collect()
}

fn simulate(elves: &mut HashSet<Coords>, rounds: usize) -> Option<usize> {
    let mut consider_directions = [Direction::N, Direction::S, Direction::W, Direction::E];

    for round in 0..rounds {
        let mut move_proposals = HashMap::new();
        let mut duplicates = HashSet::new();

        // check possible moves
        for src in elves.iter() {
            if let Some(dest) = consider(src, elves, &consider_directions) {
                if move_proposals.insert(dest, *src).is_some() {
                    duplicates.insert(dest);
                }
            }
        }

        // prune duplicates
        for d in duplicates {
            move_proposals.remove(&d);
        }

        if move_proposals.is_empty() {
            return Some(round + 1);
        }

        // execute moves
        for (dest, src) in move_proposals {
            elves.remove(&src);
            elves.insert(dest);
        }

        consider_directions.rotate_left(1);
    }

    None
}

fn consider(pos: &Coords, elves: &HashSet<Coords>, directions: &[Direction]) -> Option<Coords> {
    let neighbors = [
        pos.mov(Direction::NW),
        pos.mov(Direction::N),
        pos.mov(Direction::NE),
        pos.mov(Direction::E),
        pos.mov(Direction::SE),
        pos.mov(Direction::S),
        pos.mov(Direction::SW),
        pos.mov(Direction::W),
    ];
    if neighbors.iter().all(|n| !elves.contains(n)) {
        return None;
    }

    for direction in directions {
        let consider_fields = match direction {
            Direction::N => [&neighbors[0], &neighbors[1], &neighbors[2]],
            Direction::S => [&neighbors[4], &neighbors[5], &neighbors[6]],
            Direction::W => [&neighbors[6], &neighbors[7], &neighbors[0]],
            Direction::E => [&neighbors[2], &neighbors[3], &neighbors[4]],
            _ => unreachable!(),
        };

        if consider_fields.into_iter().all(|c| !elves.contains(c)) {
            return Some(pos.mov(*direction));
        }
    }
    None
}

fn empty_tiles_in_rectangle(elves: &HashSet<Coords>) -> usize {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for c in elves {
        x_min = x_min.min(c.x);
        x_max = x_max.max(c.x);
        y_min = y_min.min(c.y);
        y_max = y_max.max(c.y);
    }
    (x_max - x_min + 1) as usize * (y_max - y_min + 1) as usize - elves.len()
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let mut elves = parse_input(input);
        let no_move_round = simulate(&mut elves, 10);
        assert_eq!(no_move_round, None);
        format!("{}", empty_tiles_in_rectangle(&elves))
    }

    fn star2(&self, input: &str) -> String {
        let mut elves = parse_input(input);
        let no_move_round = simulate(&mut elves, usize::MAX);
        format!("{}", no_move_round.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

        let d = Day23 {};
        assert_eq!(d.star1(input), "110");
        assert_eq!(d.star2(input), "20");
    }
}
