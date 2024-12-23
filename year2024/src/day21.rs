use std::collections::{HashMap, VecDeque};

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day21 {}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        solve(input, 2).to_string()
    }

    fn star2(&self, input: &str) -> String {
        solve(input, 25).to_string()
    }
}

fn solve(input: &str, level: usize) -> usize {
    let grid_numeric: Grid2D<Numeric> = Grid2D::new_by(NUMERIC_KEYPAD, |c| c.into()).unwrap();
    let grid_directional: Grid2D<Directional> =
        Grid2D::new_by(DIRECTIONAL_KEYPAD, |c| c.into()).unwrap();

    let numeric_paths = find_all_paths(
        &grid_numeric,
        &[
            Numeric::Key0,
            Numeric::Key1,
            Numeric::Key2,
            Numeric::Key3,
            Numeric::Key4,
            Numeric::Key5,
            Numeric::Key6,
            Numeric::Key7,
            Numeric::Key8,
            Numeric::Key9,
            Numeric::KeyA,
        ],
    );
    let directional_paths = find_all_paths(
        &grid_directional,
        &[
            Directional::KeyUp,
            Directional::KeyDown,
            Directional::KeyLeft,
            Directional::KeyRight,
            Directional::KeyA,
        ],
    );

    let codes = parse_input(input);
    let mut result = 0;
    let mut cache = HashMap::new();

    for (num, code) in codes {
        let sequences_after_numeric = build_sequences(&numeric_paths, Numeric::KeyA, &code, vec![]);

        let min_len = sequences_after_numeric
            .into_iter()
            .map(|sequence| shortest_sequence_len(&directional_paths, sequence, level, &mut cache))
            .min()
            .unwrap();

        result += num * min_len;
    }

    result
}

fn parse_input(input: &str) -> Vec<(usize, Vec<Numeric>)> {
    input
        .lines()
        .map(|line| {
            let num = line[..3].parse().unwrap();
            let keys = line.chars().map(|c| c.into()).collect();

            (num, keys)
        })
        .collect()
}

const NUMERIC_KEYPAD: &str = r#"789
456
123
 0A"#;

const DIRECTIONAL_KEYPAD: &str = r#" ^A
<v>"#;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
enum Numeric {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    #[default]
    KeyInvalid,
}

impl From<char> for Numeric {
    fn from(value: char) -> Self {
        match value {
            '0' => Numeric::Key0,
            '1' => Numeric::Key1,
            '2' => Numeric::Key2,
            '3' => Numeric::Key3,
            '4' => Numeric::Key4,
            '5' => Numeric::Key5,
            '6' => Numeric::Key6,
            '7' => Numeric::Key7,
            '8' => Numeric::Key8,
            '9' => Numeric::Key9,
            'A' => Numeric::KeyA,
            _ => Numeric::KeyInvalid,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
enum Directional {
    KeyA,
    KeyDown,
    #[default]
    KeyInvalid,
    KeyLeft,
    KeyRight,
    KeyUp,
}

impl std::fmt::Display for Directional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Directional::KeyA => 'A',
            Directional::KeyDown => 'v',
            Directional::KeyInvalid => '#',
            Directional::KeyLeft => '<',
            Directional::KeyRight => '>',
            Directional::KeyUp => '^',
        };
        write!(f, "{c}")
    }
}

impl From<Direction> for Directional {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => Directional::KeyUp,
            Direction::S => Directional::KeyDown,
            Direction::W => Directional::KeyLeft,
            Direction::E => Directional::KeyRight,
            _ => Directional::KeyInvalid,
        }
    }
}

impl From<char> for Directional {
    fn from(value: char) -> Self {
        match value {
            '^' => Directional::KeyUp,
            'v' => Directional::KeyDown,
            '<' => Directional::KeyLeft,
            '>' => Directional::KeyRight,
            'A' => Directional::KeyA,
            _ => Directional::KeyInvalid,
        }
    }
}

struct SearchState {
    directions: Vec<Directional>,
    path: Vec<Coords>,
}

fn find_all_paths<T: Copy + Default + PartialEq + Eq + std::hash::Hash>(
    grid: &Grid2D<T>,
    keys: &[T],
) -> HashMap<(T, T), Vec<Vec<Directional>>> {
    let mut all_paths = HashMap::new();

    for from in keys {
        for to in keys {
            if from != to {
                let paths = find_paths(grid, *from, *to);
                all_paths.insert((*from, *to), paths);
            }
        }
    }

    all_paths
}

fn find_paths<T: Default + PartialEq>(grid: &Grid2D<T>, from: T, to: T) -> Vec<Vec<Directional>> {
    let invalid = T::default();

    let mut results = vec![];
    let mut search_states = VecDeque::new();

    let start_pos = grid.find(from).unwrap();
    let end_pos = grid.find(to).unwrap();

    search_states.push_back(SearchState {
        directions: vec![],
        path: vec![start_pos],
    });
    while let Some(search_state) = search_states.pop_front() {
        let current_pos = search_state.path.last().unwrap();
        if current_pos == &end_pos {
            results.push(search_state.directions);
            continue;
        }

        for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let neighbor_pos = current_pos.mov(dir);
            if let Some(v) = grid.at(&neighbor_pos) {
                if *v != invalid && !search_state.path.contains(&neighbor_pos) {
                    let mut new_path = search_state.path.clone();
                    new_path.push(neighbor_pos);
                    let mut new_directions = search_state.directions.clone();
                    new_directions.push(dir.into());
                    search_states.push_back(SearchState {
                        directions: new_directions,
                        path: new_path,
                    });
                }
            }
        }
    }
    retain_shortest(&mut results);

    results
}

fn retain_shortest(results: &mut Vec<Vec<Directional>>) {
    let min_len = results
        .iter()
        .map(|directions| directions.len())
        .min()
        .unwrap();
    results.retain(|directions| directions.len() == min_len);

    // remove zig-zag paths
    let min_changes = results.iter().map(|d| changes(d)).min().unwrap();
    results.retain(|d| changes(d) == min_changes);
}

fn changes(directions: &[Directional]) -> usize {
    let mut changes = 0;
    for s in directions.windows(2) {
        if s[0] != s[1] {
            changes += 1;
        }
    }
    changes
}

fn build_sequences<T: Copy + Default + PartialEq + Eq + std::hash::Hash + std::fmt::Debug>(
    moves: &HashMap<(T, T), Vec<Vec<Directional>>>,
    from_key: T,
    tail: &[T],
    path: Vec<Directional>,
) -> Vec<Vec<Directional>> {
    if tail.is_empty() {
        vec![path]
    } else {
        let to_key = tail.first().unwrap();
        let mut paths = vec![];
        if let Some(partial_paths) = moves.get(&(from_key, *to_key)) {
            for partial_path in partial_paths {
                let mut new_path = path.clone();
                new_path.extend_from_slice(partial_path);
                new_path.push(Directional::KeyA);
                paths.append(&mut build_sequences(moves, *to_key, &tail[1..], new_path));
            }
            paths
        } else {
            let mut new_path = path.clone();
            new_path.push(Directional::KeyA);
            build_sequences(moves, *to_key, &tail[1..], new_path)
        }
    }
}

fn shortest_sequence_len(
    moves: &HashMap<(Directional, Directional), Vec<Vec<Directional>>>,
    sequence: Vec<Directional>,
    level: usize,
    cache: &mut HashMap<(usize, Vec<Directional>), usize>,
) -> usize {
    if level == 0 {
        return sequence.len();
    }
    if let Some(len) = cache.get(&(level, sequence.clone())) {
        return *len;
    }

    let mut total = 0;
    for sub_key in sequence.split_inclusive(|&k| k == Directional::KeyA) {
        let sub_key_min_len = build_sequences(moves, Directional::KeyA, sub_key, vec![])
            .into_iter()
            .map(|sequence| shortest_sequence_len(moves, sequence, level - 1, cache))
            .min()
            .unwrap();
        total += sub_key_min_len;
    }
    cache.insert((level, sequence), total);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn star1() {
        let d = Day21 {};
        assert_eq!(d.star1(INPUT), "126384");
    }
}
