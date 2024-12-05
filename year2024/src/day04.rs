use common::day::Day;
use util::grid2d::{Direction, Grid2D};

pub struct Day04 {}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        format!("{}", xmas_count(&grid))
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        format!("{}", x_mas_count(&grid))
    }
}

fn xmas_count(grid: &Grid2D<char>) -> usize {
    let mut count = 0;
    let xs = grid.filter(&['X']);
    for pos in xs {
        for d in [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ] {
            let pos = pos.mov(d);
            if grid.at(&pos).unwrap_or(&'.') != &'M' {
                continue;
            }
            let pos = pos.mov(d);
            if grid.at(&pos).unwrap_or(&'.') != &'A' {
                continue;
            }
            let pos = pos.mov(d);
            if grid.at(&pos).unwrap_or(&'.') != &'S' {
                continue;
            }
            count += 1;
        }
    }
    count
}

fn x_mas_count(grid: &Grid2D<char>) -> usize {
    let mut count = 0;
    let xs = grid.filter(&['A']);
    'outer: for x in xs {
        for d in [Direction::NW, Direction::NE] {
            let pos = x.mov(d);
            let expected = match grid.at(&pos).unwrap_or(&'.') {
                'M' => 'S',
                'S' => 'M',
                _ => {
                    continue 'outer;
                }
            };
            let pos = x.mov(d.opposite());
            if grid.at(&pos).unwrap_or(&'.') != &expected {
                continue 'outer;
            }
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn star1() {
        let d = Day04 {};
        assert_eq!(d.star1(INPUT), "18");
    }

    #[test]
    fn star2() {
        let d = Day04 {};
        assert_eq!(d.star2(INPUT), "9");
    }
}
