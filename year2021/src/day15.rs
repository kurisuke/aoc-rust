use common::day::Day;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use util::grid2d::{Coords, Grid2D};

pub struct Day15 {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Coords,
    cost: u32,
    target_dist: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.target_dist)
            .cmp(&(self.cost + self.target_dist))
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Grid2D<u32> {
    Grid2D::new_by(input, |c| c.to_digit(10).unwrap()).unwrap()
}

fn neighbors(pos: &Coords, grid: &Grid2D<u32>) -> Vec<(Coords, u32)> {
    grid.neighbors_cardinal_coords(pos)
        .into_iter()
        .map(|n| grid.at(&n).map(|v| (n, *v)))
        .flatten()
        .collect()
}

fn search(init_pos: &Coords, target_pos: &Coords, grid: &Grid2D<u32>) -> Option<u32> {
    let mut frontier = BinaryHeap::new();
    let start = State {
        position: *init_pos,
        cost: 0,
        target_dist: init_pos.manhattan(target_pos) as u32,
    };
    frontier.push(start);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(*init_pos, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.position == *target_pos {
            return Some(current.cost);
        }

        for (next, next_cost) in neighbors(&current.position, grid) {
            let new_cost = cost_so_far.get(&current.position).unwrap() + next_cost;
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let target_dist = new_cost + next.manhattan(target_pos) as u32;
                frontier.push(State {
                    position: next,
                    cost: new_cost,
                    target_dist,
                });
            }
        }
    }
    None
}

fn scale_grid(grid: &Grid2D<u32>, scale: i64) -> Grid2D<u32> {
    let new_width = grid.width() * scale;
    let new_height = grid.height() * scale;

    let mut new_grid = Grid2D::with_default(
        Coords {
            x: new_width,
            y: new_height,
        },
        &0u32,
    );
    for x in 0..new_width {
        for y in 0..new_height {
            let tile_x = x / grid.width();
            let tile_y = y / grid.height();

            let offset_x = x % grid.width();
            let offset_y = y % grid.height();

            let mut v = grid
                .at(&Coords {
                    x: offset_x,
                    y: offset_y,
                })
                .unwrap()
                + tile_x as u32
                + tile_y as u32;
            if v > 9 {
                v -= 9;
            }
            new_grid.set(&Coords { x, y }, v);
        }
    }
    new_grid
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let target_pos = Coords {
            x: grid.width() - 1,
            y: grid.height() - 1,
        };
        format!(
            "{}",
            search(&Coords { x: 0, y: 0 }, &target_pos, &grid).unwrap()
        )
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);
        let scaled_grid = scale_grid(&grid, 5);
        let target_pos = Coords {
            x: scaled_grid.width() - 1,
            y: scaled_grid.height() - 1,
        };
        format!(
            "{}",
            search(&Coords { x: 0, y: 0 }, &target_pos, &scaled_grid).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

        let d = Day15 {};
        assert_eq!(d.star1(input), "40");
        assert_eq!(d.star2(input), "315");
    }
}
