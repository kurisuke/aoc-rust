use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day13 {}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|section| {
                let grid = Grid2D::new(section).unwrap();
                find_reflection(&grid, None).unwrap()
            })
            .sum::<i64>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|section| {
                let grid = Grid2D::new(section).unwrap();
                let orig_reflection = find_reflection(&grid, None).unwrap();

                for smudge_pos in grid.coords_iter() {
                    let mut grid_mod = grid.clone();
                    match grid_mod.at(&smudge_pos).unwrap() {
                        '#' => {
                            grid_mod.set(&smudge_pos, '.');
                        }
                        '.' => {
                            grid_mod.set(&smudge_pos, '#');
                        }
                        _ => unreachable!(),
                    }

                    if let Some(x) = find_reflection(&grid_mod, Some(orig_reflection)) {
                        return x;
                    }
                }
                unreachable!()
            })
            .sum::<i64>()
            .to_string()
    }
}

fn find_reflection(grid: &Grid2D<char>, skip: Option<i64>) -> Option<i64> {
    match find_reflection_1d(grid, skip) {
        Some(n) => Some(n),
        None => {
            let grid = grid.transpose();
            let skip = skip.map(|x| x / 100);
            find_reflection_1d(&grid, skip).map(|x| x * 100)
        }
    }
}

fn find_reflection_1d(grid: &Grid2D<char>, skip: Option<i64>) -> Option<i64> {
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
            if let Some(skip) = skip {
                if skip != col {
                    return Some(col);
                }
            } else {
                return Some(col);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.##..##.
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

    #[test]
    fn ex1() {
        let d = Day13 {};
        assert_eq!(d.star1(INPUT), "405");
    }

    #[test]
    fn ex2() {
        let d = Day13 {};
        assert_eq!(d.star2(INPUT), "400");
    }
}
