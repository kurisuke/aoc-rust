use std::collections::{BinaryHeap, HashMap};

use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day18 {}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let bytes = parse_input(input);
        let grid = grid_after(Coords { x: 71, y: 71 }, &bytes, 1024);
        search(&grid).unwrap().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let bytes = parse_input(input);
        let byte = first_blocker(Coords { x: 71, y: 71 }, &bytes);
        format!("{},{}", byte.x, byte.y)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Field {
    #[default]
    Safe,
    Corrupted,
}

fn parse_input(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(',');
            let x = spl.next().unwrap().parse().unwrap();
            let y = spl.next().unwrap().parse().unwrap();
            Coords { x, y }
        })
        .collect()
}

fn add_byte(grid: &mut Grid2D<Field>, bytes: &[Coords], n: usize) {
    grid.set(&bytes[n], Field::Corrupted);
}

fn grid_after(dims: Coords, bytes: &[Coords], n: usize) -> Grid2D<Field> {
    let mut grid = Grid2D::with_default(dims, &Field::Safe);
    for i in 0..n {
        add_byte(&mut grid, bytes, i);
    }
    grid
}

#[derive(PartialEq, Eq)]
struct SearchState {
    pos: Coords,
    steps: usize,
    heuristic: usize,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.steps + other.heuristic)
            .cmp(&(self.steps + self.heuristic))
            .then_with(|| (self.pos.x + self.pos.y).cmp(&(other.pos.x + other.pos.y)))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search(grid: &Grid2D<Field>) -> Option<usize> {
    let start_pos = Coords { x: 0, y: 0 };
    let end_pos = Coords {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };

    let mut search_states = BinaryHeap::new();
    let mut min_steps_for_pos = HashMap::new();

    search_states.push(SearchState {
        pos: start_pos,
        steps: 0,
        heuristic: start_pos.manhattan(&end_pos) as usize,
    });
    min_steps_for_pos.insert(start_pos, 0);

    while let Some(search_state) = search_states.pop() {
        if search_state.pos == end_pos {
            return Some(search_state.steps);
        }

        for neighbor_pos in grid.neighbors_cardinal_coords(&search_state.pos) {
            if let Some(v) = grid.at(&neighbor_pos) {
                if v == &Field::Safe {
                    let next_steps = search_state.steps + 1;

                    if &next_steps < min_steps_for_pos.get(&neighbor_pos).unwrap_or(&usize::MAX) {
                        min_steps_for_pos.insert(neighbor_pos, next_steps);
                        search_states.push(SearchState {
                            pos: neighbor_pos,
                            steps: next_steps,
                            heuristic: neighbor_pos.manhattan(&end_pos) as usize,
                        });
                    }
                }
            }
        }
    }

    None
}

fn first_blocker(dims: Coords, bytes: &[Coords]) -> Coords {
    let mut low = 0;
    let mut high = bytes.len();
    while low < high {
        let mid = (low + high) / 2;
        let grid = grid_after(dims, bytes, mid);
        if search(&grid).is_none() {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    bytes[low - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn star1() {
        let bytes = parse_input(INPUT);
        let grid = grid_after(Coords { x: 7, y: 7 }, &bytes, 12);
        assert_eq!(search(&grid), Some(22));
    }

    #[test]
    fn star2() {
        let bytes = parse_input(INPUT);
        assert_eq!(
            first_blocker(Coords { x: 7, y: 7 }, &bytes),
            Coords { x: 6, y: 1 }
        );
    }
}
