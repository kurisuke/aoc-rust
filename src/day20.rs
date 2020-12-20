use crate::day::Day;
use crate::grid2d::{Flip, Grid2D};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

pub struct Day20 {}

type Tiles = HashMap<usize, Grid2D<char>>;

struct Placement {
    length: usize,
    placement: Vec<PlacedTile>,
}

struct PlacedTile {
    tile_id: usize,
    rotation: usize,
    flip: Flip,
}

fn parse_input(input: &str) -> Tiles {
    input
        .split("\n\n")
        .map(|sec| {
            let mut it = sec.lines();
            if let Some(first_line) = it.next() {
                if first_line.len() > 0 {
                    let tile_id = scan_fmt!(first_line, "Tile {}:", usize).unwrap();
                    let grid_str: String = it.join("\n");
                    let grid = Grid2D::new(&grid_str).unwrap();
                    Some((tile_id, grid))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect()
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let length = (tiles.len() as f64).sqrt() as usize;
        format!("{}", length)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"#;
        let d = Day20 {};
        assert_eq!(d.star1(input), "3");
    }
}
