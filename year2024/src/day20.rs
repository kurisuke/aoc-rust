use std::collections::HashMap;

use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day20 {}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let track = find_track(&grid);
        let shortcuts = find_shortcuts(&track, 2);
        let result = shortcuts
            .iter()
            .filter(|(&key, _)| key >= 100)
            .map(|(_, value)| value)
            .sum::<usize>();

        result.to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);
        let track = find_track(&grid);
        let shortcuts = find_shortcuts(&track, 20);
        let result = shortcuts
            .iter()
            .filter(|(&key, _)| key >= 100)
            .map(|(_, value)| value)
            .sum::<usize>();

        result.to_string()
    }
}

#[derive(PartialEq, Eq)]
enum Field {
    Empty,
    Wall,
    Start,
    End,
}

fn parse_input(input: &str) -> Grid2D<Field> {
    Grid2D::new_by(input, |c| match c {
        '.' => Field::Empty,
        '#' => Field::Wall,
        'S' => Field::Start,
        'E' => Field::End,
        _ => unreachable!(),
    })
    .unwrap()
}

fn find_track(grid: &Grid2D<Field>) -> Vec<Coords> {
    let mut track = vec![];

    let start_pos = grid.find(Field::Start).unwrap();
    let end_pos = grid.find(Field::End).unwrap();

    track.push(start_pos);
    let mut check_pos = start_pos;
    loop {
        if check_pos == end_pos {
            break;
        }

        let mut neighbor_poses = grid.neighbors_cardinal_coords(&check_pos);
        neighbor_poses.retain(|pos| {
            grid.at(pos).unwrap() != &Field::Wall
                && (track.len() < 2 || track.get(track.len() - 2).unwrap() != pos)
        });

        assert_eq!(neighbor_poses.len(), 1);
        check_pos = neighbor_poses[0];
        track.push(check_pos);
    }

    track
}

fn find_shortcuts(track: &[Coords], max_len: usize) -> HashMap<usize, usize> {
    let mut shortcuts = HashMap::new();
    for (pos_index, pos) in track.iter().enumerate() {
        for (next_index, next_pos) in track.iter().enumerate().skip(pos_index + max_len + 1) {
            let regular_distance = next_index - pos_index;
            let shortcut_distance = pos.manhattan(next_pos) as usize;
            if shortcut_distance <= max_len && shortcut_distance < regular_distance {
                let e = shortcuts
                    .entry(regular_distance - shortcut_distance)
                    .or_insert(0);
                *e += 1;
            }
        }
    }
    shortcuts
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn star1() {
        let grid = parse_input(INPUT);
        let track = find_track(&grid);
        assert_eq!(track.len(), 85);

        let shortcuts = find_shortcuts(&track, 2);
        assert_eq!(shortcuts.get(&2), Some(&14));
        assert_eq!(shortcuts.get(&4), Some(&14));
        assert_eq!(shortcuts.get(&6), Some(&2));
        assert_eq!(shortcuts.get(&8), Some(&4));
        assert_eq!(shortcuts.get(&10), Some(&2));
        assert_eq!(shortcuts.get(&12), Some(&3));
        assert_eq!(shortcuts.get(&20), Some(&1));
        assert_eq!(shortcuts.get(&36), Some(&1));
        assert_eq!(shortcuts.get(&38), Some(&1));
        assert_eq!(shortcuts.get(&40), Some(&1));
        assert_eq!(shortcuts.get(&64), Some(&1));
    }

    #[test]
    fn star2() {
        let grid = parse_input(INPUT);
        let track = find_track(&grid);
        assert_eq!(track.len(), 85);

        let shortcuts = find_shortcuts(&track, 20);
        assert_eq!(shortcuts.get(&50), Some(&32));
        assert_eq!(shortcuts.get(&52), Some(&31));
        assert_eq!(shortcuts.get(&54), Some(&29));
        assert_eq!(shortcuts.get(&56), Some(&39));
        assert_eq!(shortcuts.get(&58), Some(&25));
        assert_eq!(shortcuts.get(&60), Some(&23));
        assert_eq!(shortcuts.get(&62), Some(&20));
        assert_eq!(shortcuts.get(&64), Some(&19));
        assert_eq!(shortcuts.get(&66), Some(&12));
        assert_eq!(shortcuts.get(&68), Some(&14));
        assert_eq!(shortcuts.get(&70), Some(&12));
        assert_eq!(shortcuts.get(&72), Some(&22));
        assert_eq!(shortcuts.get(&74), Some(&4));
        assert_eq!(shortcuts.get(&76), Some(&3));
    }
}
