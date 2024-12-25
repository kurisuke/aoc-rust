use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day25 {}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let (locks, keys) = parse_input(input);
        pairs(&locks, &keys).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

type PinHeights = Vec<i64>;

fn parse_input(input: &str) -> (Vec<PinHeights>, Vec<PinHeights>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for sec in input.split("\n\n") {
        let grid = Grid2D::new(sec).unwrap();
        let is_lock = grid.at(&Coords { x: 0, y: 0 }).unwrap() == &'#';

        if is_lock {
            locks.push(pin_heights(&grid, is_lock));
        } else {
            keys.push(pin_heights(&grid, is_lock));
        }
    }

    (locks, keys)
}

fn pin_heights(grid: &Grid2D<char>, is_lock: bool) -> PinHeights {
    let mut heights = vec![];

    for x in 0..grid.width() {
        let mut h = 0;
        if is_lock {
            while h < 5 && grid.at(&Coords { x, y: 1 + h }).unwrap() == &'#' {
                h += 1;
            }
        } else {
            while h < 5 && grid.at(&Coords { x, y: 5 - h }).unwrap() == &'#' {
                h += 1;
            }
        }
        heights.push(h);
    }

    heights
}

fn fits(lock: &PinHeights, key: &PinHeights) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
}

fn pairs(locks: &[PinHeights], keys: &[PinHeights]) -> usize {
    let mut count = 0;
    for lock in locks {
        for key in keys {
            if fits(lock, key) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn ex1() {
        let d = Day25 {};
        assert_eq!(d.star1(INPUT), "3");
    }
}
