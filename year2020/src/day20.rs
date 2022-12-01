use common::day::Day;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use util::grid2d::{Coords, Flip, Grid2D};

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
    let top_ok = if next_idx < length {
        true
    } else {
        let top_border: String = new_tile.row(0).unwrap().into_iter().collect();
        placed[next_idx - length].bottom_border == top_border
    };
    let left_ok = if next_idx % length == 0 {
        true
    } else {
        let left_border: String = new_tile.col(0).unwrap().into_iter().collect();
        placed[next_idx - 1].right_border == left_border
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
            for flip in [Flip::FlipNone, Flip::FlipH].iter() {
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
        .filter_map(|sec| {
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
            let e = border_to_tiles.entry(border).or_insert_with(Vec::new);
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

fn assemble(tiles: &Tiles) -> Vec<PlacedTile> {
    let length = (tiles.len() as f64).sqrt() as usize;
    let placed = vec![];

    let unplaced: HashSet<_> = tiles.keys().copied().collect();
    let (corner_tiles, edge_tiles) = find_corners_borders(tiles);
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

    place_next(tiles, length, placed, unplaced).unwrap()
}

fn merge(tiles: &Tiles, placement: Vec<PlacedTile>) -> Grid2D<char> {
    let tile_length = (placement.len() as f64).sqrt() as i64;
    let grid_length = tiles.values().next().unwrap().width();
    let big_length = tile_length * (grid_length - 2);

    let mut big_pic = Grid2D::with_default(
        Coords {
            x: big_length,
            y: big_length,
        },
        &' ',
    );
    for (idx, placed_tile) in placement.iter().enumerate() {
        let row = idx as i64 / tile_length * (grid_length - 2);
        let col = idx as i64 % tile_length * (grid_length - 2);

        let grid = tiles.get(&placed_tile.tile_id).unwrap();
        let clipped = grid
            .clip(
                Coords { x: 1, y: 1 },
                Coords {
                    x: grid_length - 1,
                    y: grid_length - 1,
                },
            )
            .unwrap();

        let flipped = clipped.flip(placed_tile.flip);
        let times_rot = placed_tile.rotation / 90;
        let mut rotated = flipped;
        for _ in 0..times_rot {
            rotated = rotated.rotate90();
        }
        big_pic.paste(Coords { x: col, y: row }, &rotated);
    }

    big_pic
}

fn match_pattern(grid: &mut Grid2D<char>, pattern: &[Coords]) -> usize {
    let pattern_x_max = pattern.iter().map(|c| c.x).max().unwrap();
    let pattern_y_max = pattern.iter().map(|c| c.y).max().unwrap();

    let mut num_found = 0;

    for y in 0..(grid.width() - pattern_y_max) {
        for x in 0..(grid.width() - pattern_x_max) {
            let is_match = pattern.iter().all(|c| {
                grid.at(&Coords {
                    x: x + c.x,
                    y: y + c.y,
                })
                .unwrap()
                    == &'#'
            });
            if is_match {
                num_found += 1;
                for c in pattern {
                    grid.set(
                        &Coords {
                            x: x + c.x,
                            y: y + c.y,
                        },
                        'O',
                    );
                }
            }
        }
    }

    num_found
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let (corner_tiles, _) = find_corners_borders(&tiles);
        format!("{}", corner_tiles.iter().product::<usize>())
    }

    fn star2(&self, input: &str) -> String {
        let tiles = parse_input(input);
        let full_placement = assemble(&tiles);
        let merged = merge(&tiles, full_placement);

        let pattern = vec![
            Coords { y: 0, x: 18 },
            Coords { y: 1, x: 0 },
            Coords { y: 1, x: 5 },
            Coords { y: 1, x: 6 },
            Coords { y: 1, x: 11 },
            Coords { y: 1, x: 12 },
            Coords { y: 1, x: 17 },
            Coords { y: 1, x: 18 },
            Coords { y: 1, x: 19 },
            Coords { y: 2, x: 1 },
            Coords { y: 2, x: 4 },
            Coords { y: 2, x: 7 },
            Coords { y: 2, x: 10 },
            Coords { y: 2, x: 13 },
            Coords { y: 2, x: 16 },
        ];

        for mut t in merged.transformations() {
            if match_pattern(&mut t, &pattern) > 0 {
                return format!("{}", t.count('#'));
            }
        }
        String::from("err")
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
        assert_eq!(d.star2(input), "273");
    }
}
