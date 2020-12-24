use crate::day::Day;
use std::collections::HashSet;

pub struct Day24 {}

type HexCoord = (i32, i32);

const NEIGHBORS_PLUS_SELF: [(i32, i32); 7] =
    [(0, 0), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];
const NEIGHBORS: [(i32, i32); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

fn get_direction(dir_str: &str) -> Option<HexCoord> {
    match dir_str {
        "e" => Some((1, 0)),
        "se" => Some((0, 1)),
        "sw" => Some((-1, 1)),
        "w" => Some((-1, 0)),
        "nw" => Some((0, -1)),
        "ne" => Some((1, -1)),
        _ => None,
    }
}

fn parse_input(input: &str) -> Vec<Vec<HexCoord>> {
    let dir_strs = vec!["e", "se", "sw", "w", "nw", "ne"];
    input
        .lines()
        .map(|mut line| {
            let mut line_vec = vec![];
            while !line.is_empty() {
                for d in dir_strs.iter() {
                    if line.starts_with(d) {
                        line_vec.push(get_direction(d).unwrap());
                        line = &line[d.len()..];
                        break;
                    }
                }
            }
            line_vec
        })
        .collect()
}

fn init_layout(paths: Vec<Vec<HexCoord>>) -> HashSet<HexCoord> {
    let mut black_tiles = HashSet::new();
    for path in paths {
        let dest = path
            .iter()
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        if black_tiles.contains(&dest) {
            black_tiles.remove(&dest);
        } else {
            black_tiles.insert(dest);
        }
    }
    black_tiles
}

fn evolve(tiles: HashSet<HexCoord>) -> HashSet<HexCoord> {
    let mut new_tiles = HashSet::new();
    let mut checked_tiles = HashSet::new();
    for tile in tiles.iter() {
        for active in NEIGHBORS_PLUS_SELF
            .iter()
            .map(|n| (tile.0 + n.0, tile.1 + n.1))
        {
            if checked_tiles.insert(active) {
                let num_black_neighbors = NEIGHBORS
                    .iter()
                    .filter(|n| tiles.contains(&(active.0 + n.0, active.1 + n.1)))
                    .count();
                if tiles.contains(&active) {
                    // black
                    if num_black_neighbors == 1 || num_black_neighbors == 2 {
                        new_tiles.insert(active);
                    }
                } else {
                    // white
                    if num_black_neighbors == 2 {
                        new_tiles.insert(active);
                    }
                }
            }
        }
    }
    new_tiles
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let paths = parse_input(input);
        let black_tiles = init_layout(paths);
        format!("{}", black_tiles.len())
    }

    fn star2(&self, input: &str) -> String {
        let paths = parse_input(input);
        let mut black_tiles = init_layout(paths);
        for _ in 1..=100 {
            black_tiles = evolve(black_tiles);
        }
        format!("{}", black_tiles.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let d = Day24 {};
        assert_eq!(d.star1(input), "10");
        assert_eq!(d.star2(input), "2208");
    }
}
