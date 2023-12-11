use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day11 {}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let coords = galaxy_coords(&grid, 1);
        sum_shortest_paths(&coords).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let coords = galaxy_coords(&grid, 999_999);
        sum_shortest_paths(&coords).to_string()
    }
}

fn galaxy_coords(grid: &Grid2D<char>, spread: i64) -> Vec<Coords> {
    let coords = grid.filter(&['#']);

    let mut empty_rows = vec![];
    for y in 0..grid.height() {
        if grid.row(y).unwrap().iter().all(|c| **c == '.') {
            empty_rows.push(y);
        }
    }

    let mut empty_cols = vec![];
    for x in 0..grid.height() {
        if grid.col(x).unwrap().iter().all(|c| **c == '.') {
            empty_cols.push(x);
        }
    }
    coords
        .into_iter()
        .map(|coord| {
            let mut actual_coord = coord;
            for row in &empty_rows {
                if *row < coord.y {
                    actual_coord.y += spread;
                } else {
                    break;
                }
            }
            for col in &empty_cols {
                if *col < coord.x {
                    actual_coord.x += spread;
                } else {
                    break;
                }
            }
            actual_coord
        })
        .collect()
}

fn sum_shortest_paths(coords: &[Coords]) -> u64 {
    let mut sum = 0;

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            sum += coords[i].manhattan(&coords[j]);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn star1() {
        let d = Day11 {};
        assert_eq!(d.star1(INPUT), "374");
    }

    #[test]
    fn star2() {
        let grid = Grid2D::new(INPUT).unwrap();

        let coords = galaxy_coords(&grid, 9);
        assert_eq!(sum_shortest_paths(&coords), 1030);

        let coords = galaxy_coords(&grid, 99);
        assert_eq!(sum_shortest_paths(&coords), 8410);
    }
}
