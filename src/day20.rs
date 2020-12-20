use crate::day::Day;
use crate::grid2d::{Flip, Grid2D};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

pub struct Day20 {}

type Tiles = HashMap<usize, Grid2D<char>>;

#[derive(Clone, Debug)]
struct PlacedTile {
    tile_id: usize,
    rotation: usize,
    flip: Flip,
    right_border: String,
    bottom_border: String,
}

fn fits_with_neighbors(length: usize, placed: &[PlacedTile], new_tile: &Grid2D<char>) -> bool {
    let next_idx = placed.len();
    let left_neighbor_idx = if next_idx % length == 0 {
        None
    } else {
        Some(next_idx - 1)
    };
    let top_neighbor_idx = if next_idx < length {
        None
    } else {
        Some(next_idx - length)
    };

    let top_ok = if let Some(top_neighbor_idx) = top_neighbor_idx {
        let top_border: String = new_tile.row(0).unwrap().into_iter().collect();
        placed[top_neighbor_idx].bottom_border == top_border
    } else {
        true
    };

    let left_ok = if let Some(left_neighbor_idx) = left_neighbor_idx {
        let left_border: String = new_tile.col(0).unwrap().into_iter().collect();
        placed[left_neighbor_idx].right_border == left_border
    } else {
        true
    };

    top_ok && left_ok
}

fn place_next(
    tiles: &Tiles,
    length: usize,
    placed: Vec<PlacedTile>,
    unplaced: HashSet<usize>,
) -> Option<Vec<PlacedTile>> {
    if unplaced.is_empty() {
        Some(placed)
    } else {
        for tile_id in unplaced.iter() {
            let grid = tiles.get(tile_id).unwrap();
            for flip in [Flip::FlipNone, Flip::FlipH, Flip::FlipV].iter() {
                let mut flipped = grid.flip(*flip);
                for rotation in 0..4 {
                    if fits_with_neighbors(length, &placed, &flipped) {
                        let mut new_placed = placed.clone();
                        let new_tile = PlacedTile {
                            tile_id: *tile_id,
                            rotation: rotation * 90,
                            flip: *flip,
                            right_border: flipped
                                .col(flipped.width() as i64 - 1)
                                .unwrap()
                                .into_iter()
                                .collect(),
                            bottom_border: flipped
                                .row(flipped.height() as i64 - 1)
                                .unwrap()
                                .into_iter()
                                .collect(),
                        };
                        new_placed.push(new_tile);

                        let mut new_unplaced = unplaced.clone();
                        new_unplaced.remove(tile_id);

                        if let Some(x) = place_next(tiles, length, new_placed, new_unplaced) {
                            return Some(x);
                        }
                    }
                    flipped = flipped.rotate90();
                }
            }
        }
        None
    }
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
        let placed = vec![];
        let unplaced: HashSet<_> = tiles.keys().copied().collect();
        let full_placement = place_next(&tiles, length, placed, unplaced).unwrap();
        let top_left = full_placement[0].tile_id;
        let top_right = full_placement[length - 1].tile_id;
        let bottom_left = full_placement[(length - 1) * length].tile_id;
        let bottom_right = full_placement[length * length - 1].tile_id;
        format!("{}", top_left * top_right * bottom_left * bottom_right)
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
        assert_eq!(d.star1(input), "20899048083289");
    }
}
