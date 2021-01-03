use crate::day::Day;
use crate::util::grid2d::{Coords, Grid2D};
use crate::util::knothash::KnotHash;
use std::collections::VecDeque;

pub struct Day14 {}

fn convert_to_grid(hashes: &[KnotHash]) -> Grid2D<char> {
    let mut grid = Grid2D::with_default(Coords { x: 128, y: 128 }, &'.');
    for (y, hash) in hashes.iter().enumerate() {
        for (i, byte) in hash.bytes().enumerate() {
            for j in 0..8 {
                let v = if byte & (1 << j) > 0 { '#' } else { '.' };
                let x = i * 8 + (7 - j);
                grid.set(
                    &Coords {
                        x: x as i64,
                        y: y as i64,
                    },
                    v,
                );
            }
        }
    }
    grid
}

fn calc_hashes(prefix: &str) -> Vec<KnotHash> {
    (0..128)
        .map(|i| {
            let hash_string = format!("{}-{}", prefix, i);
            KnotHash::from(&hash_string)
        })
        .collect()
}

fn fill_region(grid: &mut Grid2D<char>, start: &Coords) {
    let mut frontier = VecDeque::new();
    frontier.push_back(*start);
    grid.set(start, 'F');

    while let Some(cur) = frontier.pop_front() {
        for neighbor in grid.neighbors_cardinal_coords(&cur).iter() {
            if let Some(&'#') = grid.at(neighbor) {
                frontier.push_back(*neighbor);
                grid.set(neighbor, 'F');
            }
        }
    }
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let hashes = calc_hashes(input.trim());
        let used_squares = hashes
            .iter()
            .map(|hash| hash.bytes().map(|x| x.count_ones()).sum::<u32>())
            .sum::<u32>();
        format!("{}", used_squares)
    }

    fn star2(&self, input: &str) -> String {
        let hashes = calc_hashes(input.trim());
        let mut grid = convert_to_grid(&hashes);
        let all_coords: Vec<_> = grid.coords_iter().collect();
        let regions = all_coords
            .iter()
            .filter(|c| {
                if grid.at(&c).unwrap() == &'#' {
                    fill_region(&mut grid, &c);
                    true
                } else {
                    false
                }
            })
            .count();
        format!("{}", regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day14 {};
        assert_eq!(d.star1("flqrgnkx"), "8108");
        assert_eq!(d.star2("flqrgnkx"), "1242");
    }
}
