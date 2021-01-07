use common::day::Day;
use util::grid2d::{Coords, Grid2D, Wrap};

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let num_trees = grid
            .traverse_wrap(&Coords { x: 3, y: 1 }, Wrap::WrapX)
            .filter(|&&c| c == '#')
            .count();
        format!("{:?}", num_trees)
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let slopes = vec![
            Coords { x: 1, y: 1 },
            Coords { x: 3, y: 1 },
            Coords { x: 5, y: 1 },
            Coords { x: 7, y: 1 },
            Coords { x: 1, y: 2 },
        ];
        let prod: usize = slopes
            .iter()
            .map(|s| {
                grid.traverse_wrap(s, Wrap::WrapX)
                    .filter(|&&c| c == '#')
                    .count()
            })
            .product();
        format!("{:?}", prod)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let d = Day03 {};
        assert_eq!(d.star1(input), "7");
        assert_eq!(d.star2(input), "336");
    }
}
