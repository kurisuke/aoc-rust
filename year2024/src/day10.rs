use std::collections::{HashSet, VecDeque};

use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |c| c.to_digit(10).unwrap() as u8).unwrap();
        let score = grid
            .filter(&[0])
            .into_iter()
            .map(|trailhead| trail_score(&grid, trailhead, false))
            .sum::<usize>();
        format!("{}", score)
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |c| c.to_digit(10).unwrap() as u8).unwrap();
        let rating = grid
            .filter(&[0])
            .into_iter()
            .map(|trailhead| trail_score(&grid, trailhead, true))
            .sum::<usize>();
        format!("{}", rating)
    }
}

fn trail_score(grid: &Grid2D<u8>, trailhead: Coords, all_paths: bool) -> usize {
    let mut score = 0;
    let mut frontier = VecDeque::new();
    frontier.push_back(trailhead);
    let mut visited = HashSet::new();
    if !all_paths {
        visited.insert(trailhead);
    }

    while let Some(pos) = frontier.pop_front() {
        let pos_height = grid.at(&pos).unwrap();
        for neighbor_pos in grid.neighbors_cardinal_coords(&pos) {
            if let Some(neighbor_height) = grid.at(&neighbor_pos) {
                if *neighbor_height == pos_height + 1 && (all_paths || visited.insert(neighbor_pos))
                {
                    if *neighbor_height == 9 {
                        score += 1;
                    } else {
                        frontier.push_back(neighbor_pos);
                    }
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn star1() {
        let d = Day10 {};
        assert_eq!(d.star1(INPUT), "36");
    }

    #[test]
    fn star2() {
        let d = Day10 {};
        assert_eq!(d.star2(INPUT), "81");
    }
}
