use std::collections::{BinaryHeap, HashMap, HashSet};

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day16 {}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        search(&grid).0.unwrap().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);
        search(&grid).1.len().to_string()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
struct State {
    pos: Coords,
    orientation: Direction,
}

#[derive(PartialEq, Eq, Debug)]
struct SearchState {
    state: State,
    score: usize,
    path: Vec<Coords>,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
enum Field {
    Start,
    End,
    Empty,
    Wall,
}

fn parse_input(input: &str) -> Grid2D<Field> {
    Grid2D::new_by(input, |c| match c {
        'S' => Field::Start,
        'E' => Field::End,
        '#' => Field::Wall,
        '.' => Field::Empty,
        _ => unreachable!(),
    })
    .unwrap()
}

fn search(grid: &Grid2D<Field>) -> (Option<usize>, HashSet<Coords>) {
    let start_pos = grid.find(Field::Start).unwrap();
    let end_pos = grid.find(Field::End).unwrap();

    let mut search_states = BinaryHeap::new();
    let mut min_cost_for_state = HashMap::new();

    let mut best_path_score = None;
    let mut on_best_path = HashSet::new();

    let init_state = State {
        pos: start_pos,
        orientation: Direction::E,
    };

    search_states.push(SearchState {
        state: init_state,
        score: 0,
        path: vec![start_pos],
    });
    min_cost_for_state.insert(init_state, 0);

    while let Some(search_state) = search_states.pop() {
        // println!("check state: {:?}", search_state);
        if search_state.state.pos == end_pos {
            if let Some(score) = best_path_score {
                if score < search_state.score {
                    return (best_path_score, on_best_path);
                } else {
                    on_best_path.extend(search_state.path);
                }
            } else {
                best_path_score = Some(search_state.score);
                on_best_path.extend(search_state.path);
            }
            continue;
        }

        for next in next_states(grid, &search_state) {
            if &next.score <= min_cost_for_state.get(&next.state).unwrap_or(&usize::MAX) {
                min_cost_for_state.insert(next.state, next.score);
                search_states.push(next);
            }
        }
    }

    (None, on_best_path)
}

fn next_states(grid: &Grid2D<Field>, search_state: &SearchState) -> Vec<SearchState> {
    let mut next = vec![];

    // forward
    let forward_pos = search_state.state.pos.mov(search_state.state.orientation);
    if grid.at(&forward_pos).unwrap() != &Field::Wall {
        let mut next_path = search_state.path.clone();
        next_path.push(forward_pos);
        next.push(SearchState {
            state: State {
                pos: forward_pos,
                orientation: search_state.state.orientation,
            },
            score: search_state.score + 1,
            path: next_path,
        });
    }

    // turn left / right
    for orientation in [
        search_state.state.orientation.left90(),
        search_state.state.orientation.right90(),
    ] {
        next.push(SearchState {
            state: State {
                pos: search_state.state.pos,
                orientation,
            },
            score: search_state.score + 1000,
            path: search_state.path.clone(),
        });
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const INPUT2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn star1() {
        let d = Day16 {};
        assert_eq!(d.star1(INPUT1), "7036");
        assert_eq!(d.star1(INPUT2), "11048");
    }

    #[test]
    fn star2() {
        let d = Day16 {};
        assert_eq!(d.star2(INPUT1), "45");
        assert_eq!(d.star2(INPUT2), "64");
    }
}
