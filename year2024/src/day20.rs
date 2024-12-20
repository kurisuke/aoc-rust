use std::collections::HashMap;

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day20 {}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let track = find_track(&grid);
        let shortcuts = find_shortcuts(&grid, &track);
        let result = shortcuts
            .iter()
            .filter(|(&key, _)| key >= 100)
            .map(|(_, value)| value)
            .sum::<usize>();

        result.to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
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

fn find_shortcuts(grid: &Grid2D<Field>, track: &[Coords]) -> HashMap<usize, usize> {
    let mut shortcuts = HashMap::new();
    for (pos_index, pos) in track.iter().enumerate() {
        for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let skip_pos = pos.mov(d);
            let next_pos = skip_pos.mov(d);
            if let (Some(skip_field), Some(next_field)) = (grid.at(&skip_pos), grid.at(&next_pos)) {
                if skip_field == &Field::Wall && next_field != &Field::Wall {
                    let next_index = track
                        .iter()
                        .position(|&track_pos| track_pos == next_pos)
                        .unwrap();

                    if pos_index < next_index {
                        let saved_time = next_index - pos_index - 2;
                        let e = shortcuts.entry(saved_time).or_insert(0);
                        *e += 1;
                    }
                }
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
    fn ex1() {
        let grid = parse_input(INPUT);
        let track = find_track(&grid);
        assert_eq!(track.len(), 85);

        let shortcuts = find_shortcuts(&grid, &track);
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
}
