use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day13 {}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|section| {
                let grid = Grid2D::new(section).unwrap();

                match find_reflection(&grid) {
                    Some(n) => n,
                    None => {
                        let grid = grid.transpose();
                        find_reflection(&grid).unwrap() * 100
                    }
                }
            })
            .sum::<i64>()
            .to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn find_reflection(grid: &Grid2D<char>) -> Option<i64> {
    for col in 1..grid.width() {
        let mut col_left = col - 1;
        let mut col_right = col;
        let mut fulfilled = true;
        loop {
            fulfilled &= (0..grid.height()).all(|row| {
                grid.at(&Coords {
                    y: row,
                    x: col_left,
                }) == grid.at(&Coords {
                    y: row,
                    x: col_right,
                })
            });
            if !fulfilled || col_left == 0 || col_right == grid.width() - 1 {
                break;
            }
            col_left -= 1;
            col_right += 1;
        }
        if fulfilled {
            return Some(col);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let d = Day13 {};
        assert_eq!(d.star1(input), "405");
    }
}
