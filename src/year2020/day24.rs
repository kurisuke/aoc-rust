use crate::day::Day;
use std::collections::HashSet;
use crate::util::hex::HexCoord;

pub const NEIGHBORS_PLUS_SELF: [HexCoord; 7] =
    [HexCoord (0, 0), HexCoord (1, 0), HexCoord (0, 1), HexCoord (-1, 1), HexCoord (-1, 0), HexCoord (0, -1), HexCoord (1, -1)];
pub const NEIGHBORS: [HexCoord; 6] = [HexCoord (1, 0), HexCoord (0, 1), HexCoord (-1, 1), HexCoord (-1, 0), HexCoord (0, -1), HexCoord (1, -1)];

pub struct Day24 {}

fn parse_input(input: &str) -> Vec<Vec<HexCoord>> {
    let dir_strs = vec!["e", "se", "sw", "w", "nw", "ne"];
    input
        .lines()
        .map(|mut line| {
            let mut line_vec = vec![];
            while !line.is_empty() {
                for d in dir_strs.iter() {
                    if line.starts_with(d) {
                        line_vec.push(HexCoord::direction_pointy(d).unwrap());
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
            .fold(HexCoord(0, 0), |acc, x| acc + *x);
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
            .map(|n| *tile + *n)
        {
            if checked_tiles.insert(active) {
                let num_black_neighbors = NEIGHBORS
                    .iter()
                    .filter(|n| tiles.contains(&(active + **n)))
                    .count();
                if num_black_neighbors == 2 || (num_black_neighbors == 1 && tiles.contains(&active))
                {
                    new_tiles.insert(active);
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
