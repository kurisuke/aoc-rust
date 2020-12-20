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

#[derive(Clone)]
struct Unplaced {
    corners: HashSet<usize>,
    edges: HashSet<usize>,
    rest: HashSet<usize>,
}

enum Position {
    Corner,
    Edge,
    Rest,
}

fn get_position(pos: usize, length: usize) -> Position {
    let row = pos / length;
    let col = pos % length;
    if (row == 0 || row == length - 1) && (col == 0 || col == length - 1) {
        Position::Corner
    } else if row == 0 || row == length - 1 || col == 0 || col == length - 1 {
        Position::Edge
    } else {
        Position::Rest
    }
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
    unplaced: Unplaced,
) -> Option<Vec<PlacedTile>> {
    if unplaced.corners.is_empty() {
        Some(placed)
    } else {
        let unplaced_type = match get_position(placed.len(), length) {
            Position::Corner => unplaced.corners.clone(),
            Position::Edge => unplaced.edges.clone(),
            Position::Rest => unplaced.rest.clone(),
        };
        for tile_id in unplaced_type.iter() {
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
                        match get_position(placed.len(), length) {
                            Position::Corner => {
                                new_unplaced.corners.remove(tile_id);
                            }
                            Position::Edge => {
                                new_unplaced.edges.remove(tile_id);
                            }
                            Position::Rest => {
                                new_unplaced.rest.remove(tile_id);
                            }
                        }

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
                if !first_line.is_empty() {
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

fn get_borders(grid: &Grid2D<char>) -> Vec<String> {
    let t: String = grid.row(0).unwrap().into_iter().collect();
    let b: String = grid.row(grid.height() - 1).unwrap().into_iter().collect();
    let l: String = grid.col(0).unwrap().into_iter().collect();
    let r: String = grid.col(grid.width() - 1).unwrap().into_iter().collect();
    let t_rev = t.chars().rev().collect::<String>();
    let b_rev = b.chars().rev().collect::<String>();
    let l_rev = l.chars().rev().collect::<String>();
    let r_rev = r.chars().rev().collect::<String>();
    vec![t, b, l, r, t_rev, b_rev, l_rev, r_rev]
}

fn find_corners_borders(tiles: &Tiles) -> (HashSet<usize>, HashSet<usize>) {
    // initialize map: border strings to tile_ids
    let mut border_to_tiles = HashMap::new();
    for (tile_id, grid) in tiles {
        for border in get_borders(grid) {
            let e = border_to_tiles.entry(border).or_insert(vec![]);
            e.push(tile_id);
        }
    }

    let mut corner_tiles = HashSet::new();
    let mut edge_tiles = HashSet::new();
    for (tile_id, grid) in tiles {
        let mut match_cnt = 0;
        for border in get_borders(grid) {
            let e = border_to_tiles.get(&border).unwrap();
            if e.len() > 1 {
                match_cnt += 1;
            }
        }
        if match_cnt == 4 {
            // connections only for 2 sides
            corner_tiles.insert(*tile_id);
        } else if match_cnt == 6 {
            // connections for 3 sides
            edge_tiles.insert(*tile_id);
        }
    }

    (corner_tiles, edge_tiles)
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let length = (tiles.len() as f64).sqrt() as usize;
        let placed = vec![];

        let unplaced: HashSet<_> = tiles.keys().copied().collect();
        let (corner_tiles, edge_tiles) = find_corners_borders(&tiles);
        assert_eq!(corner_tiles.len(), 4);
        assert_eq!(edge_tiles.len(), 4 * length - 8);
        let rest_tiles: HashSet<_> = unplaced.difference(&corner_tiles).cloned().collect();
        let rest_tiles: HashSet<_> = rest_tiles.difference(&edge_tiles).cloned().collect();
        assert_eq!(rest_tiles.len(), (length - 2) * (length - 2));
        let unplaced = Unplaced {
            corners: corner_tiles,
            edges: edge_tiles,
            rest: rest_tiles,
        };

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
