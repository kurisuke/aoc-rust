use std::collections::HashMap;

use common::day::Day;
use util::grid2d::{Direction, Grid2D};

pub struct Day14 {}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        slide_direction(&mut grid, Direction::N);
        load(&grid).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let mut loads = HashMap::new();
        let mut i = 0;
        loads.insert(grid.clone(), 0);
        loop {
            i += 1;
            slide_cycle(&mut grid);
            if loads.contains_key(&grid) {
                let offset = loads.get(&grid).unwrap();
                let length = i - loads.get(&grid).unwrap();
                let target = 1000000000;
                let target_index = offset + (target - offset) % length;
                return load(
                    loads
                        .iter()
                        .find(|(_, x)| x == &&target_index)
                        .map(|(grid, _)| grid)
                        .unwrap(),
                )
                .to_string();
            } else {
                loads.insert(grid.clone(), i);
            }
        }
    }
}

fn slide_cycle(grid: &mut Grid2D<char>) {
    slide_direction(grid, Direction::N);
    slide_direction(grid, Direction::W);
    slide_direction(grid, Direction::S);
    slide_direction(grid, Direction::E);
}

fn slide_direction(grid: &mut Grid2D<char>, dir: Direction) {
    match dir {
        Direction::N => {
            for i in 0..grid.width() {
                grid.set_col(i, slide_col(grid.col(i).unwrap(), true));
            }
        }
        Direction::S => {
            for i in 0..grid.width() {
                grid.set_col(i, slide_col(grid.col(i).unwrap(), false));
            }
        }
        Direction::W => {
            for i in 0..grid.height() {
                grid.set_row(i, slide_row(grid.row(i).unwrap(), true));
            }
        }
        Direction::E => {
            for i in 0..grid.width() {
                grid.set_row(i, slide_row(grid.row(i).unwrap(), false));
            }
        }
        _ => unreachable!(),
    }
}

fn slide_col(col: Vec<&char>, north: bool) -> Vec<char> {
    let mut col_new = vec![];
    for range in col.split(|x| x == &&'#') {
        let num_movable = range.iter().filter(|x| x == &&&'O').count();
        if north {
            col_new.extend(std::iter::repeat_n('O', num_movable));
            col_new.extend(std::iter::repeat_n('.', range.len() - num_movable));
            col_new.push('#');
        } else {
            col_new.extend(std::iter::repeat_n('.', range.len() - num_movable));
            col_new.extend(std::iter::repeat_n('O', num_movable));
            col_new.push('#');
        }
    }
    col_new.pop();
    col_new
}

fn slide_row(row: Vec<&char>, west: bool) -> Vec<char> {
    let mut row_new = vec![];
    for range in row.split(|x| x == &&'#') {
        let num_movable = range.iter().filter(|x| x == &&&'O').count();
        if west {
            row_new.extend(std::iter::repeat_n('O', num_movable));
            row_new.extend(std::iter::repeat_n('.', range.len() - num_movable));
            row_new.push('#');
        } else {
            row_new.extend(std::iter::repeat_n('.', range.len() - num_movable));
            row_new.extend(std::iter::repeat_n('O', num_movable));
            row_new.push('#');
        }
    }
    row_new.pop();
    row_new
}

fn load(grid: &Grid2D<char>) -> usize {
    (0..grid.height())
        .map(|i| {
            grid.row(i)
                .unwrap()
                .into_iter()
                .filter(|x| x == &&'O')
                .count()
                * (grid.height() - i) as usize
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn ex1() {
        let d = Day14 {};
        assert_eq!(d.star1(INPUT), "136");
    }

    #[test]
    fn ex2() {
        let d = Day14 {};
        assert_eq!(d.star2(INPUT), "64");
    }
}
