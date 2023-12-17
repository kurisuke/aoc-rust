use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day17 {}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |x| x.to_digit(10).unwrap() as usize).unwrap();
        search(&grid, 0, 3).unwrap().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |x| x.to_digit(10).unwrap() as usize).unwrap();
        search(&grid, 4, 10).unwrap().to_string()
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct SearchState {
    heat_loss: usize,
    pos: Coords,
    direction: Direction,
    straight_steps: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct HashState {
    pos: Coords,
    direction: Direction,
    straight_steps: usize,
}

impl HashState {
    fn from(state: &SearchState) -> HashState {
        HashState {
            pos: state.pos,
            direction: state.direction,
            straight_steps: state.straight_steps,
        }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| (self.pos.x + self.pos.y).cmp(&(other.pos.x + other.pos.y)))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(
    grid: &Grid2D<usize>,
    min_before_turn_or_stop: usize,
    max_before_turn: usize,
) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();

    let end_pos = Coords {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };

    for d_init in [Direction::E, Direction::S] {
        let init = SearchState {
            heat_loss: 0,
            pos: Coords { x: 0, y: 0 },
            direction: d_init,
            straight_steps: 1,
        };
        visited.insert(HashState::from(&init), init.heat_loss);
        queue.push(init);
    }

    while let Some(state) = queue.pop() {
        // println!("state: {:?}", state);
        if state.pos == end_pos && state.straight_steps >= min_before_turn_or_stop {
            return Some(state.heat_loss);
        }

        let new_dirs = [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter(|d| {
                d != &state.direction.opposite()
                    && !(d != &state.direction && state.straight_steps < min_before_turn_or_stop)
                    && !(d == &state.direction && state.straight_steps >= max_before_turn)
            });

        let new_states = new_dirs
            .into_iter()
            .filter(|d| grid.at(&state.pos.mov(*d)).is_some())
            .map(|d| {
                let new_pos = state.pos.mov(d);
                SearchState {
                    heat_loss: state.heat_loss + grid.at(&new_pos).unwrap(),
                    pos: new_pos,
                    direction: d,
                    straight_steps: if d == state.direction {
                        state.straight_steps + 1
                    } else {
                        1
                    },
                }
            });

        for new_state in new_states {
            let new_hash_state = HashState::from(&new_state);
            if !visited.contains_key(&new_hash_state)
                || new_state.heat_loss < visited[&new_hash_state]
            {
                visited.insert(new_hash_state, new_state.heat_loss);
                queue.push(new_state);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn ex1() {
        let d = Day17 {};
        assert_eq!(d.star1(INPUT), "102");
    }
    #[test]

    fn ex2() {
        let d = Day17 {};
        assert_eq!(d.star2(INPUT), "94");
    }

    #[test]
    fn ex3() {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

        let d = Day17 {};
        assert_eq!(d.star2(input), "71");
    }
}
