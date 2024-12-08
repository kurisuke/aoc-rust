use std::collections::HashMap;

use common::day::Day;
use itertools::Itertools;
use util::grid2d::{Coords, Grid2D};

pub struct Day08 {}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let (antennas, grid) = parse_input(input);
        let mut antinodes = Grid2D::with_default(grid.dimensions(), &false);

        for antenna in antennas.values() {
            mark_antinodes(&mut antinodes, antenna);
        }

        format!("{}", antinodes.count(true))
    }

    fn star2(&self, input: &str) -> String {
        let (antennas, grid) = parse_input(input);
        let mut antinodes = Grid2D::with_default(grid.dimensions(), &false);

        for antenna in antennas.values() {
            mark_antinodes_2(&mut antinodes, antenna);
        }

        format!("{}", antinodes.count(true))
    }
}

type Antennas = HashMap<char, Vec<Coords>>;

fn parse_input(input: &str) -> (Antennas, Grid2D<char>) {
    let mut antennas = HashMap::new();
    let grid = Grid2D::new(input).unwrap();
    for pos in grid.coords_iter() {
        let v = grid.at(&pos).unwrap();
        if v != &'.' {
            let e = antennas.entry(*v).or_insert(vec![]);
            e.push(pos);
        }
    }

    (antennas, grid)
}

fn mark_antinodes(antinodes: &mut Grid2D<bool>, antenna: &[Coords]) {
    for a in antenna.iter().combinations(2) {
        let v = *a[1] - *a[0];
        let antinode1 = *a[0] - v;
        let antinode2 = *a[1] + v;
        let _ = antinodes.set(&antinode1, true);
        let _ = antinodes.set(&antinode2, true);
    }
}

fn mark_antinodes_2(antinodes: &mut Grid2D<bool>, antenna: &[Coords]) {
    for a in antenna.iter().combinations(2) {
        let v = *a[1] - *a[0];

        let mut antinode = *a[0];
        while antinodes.set(&antinode, true) {
            antinode -= v;
        }
        let mut antinode = *a[1];
        while antinodes.set(&antinode, true) {
            antinode += v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn star1() {
        let d = Day08 {};
        assert_eq!(d.star1(INPUT), "14");
    }

    #[test]
    fn star2() {
        let input2 = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;

        let d = Day08 {};
        assert_eq!(d.star2(input2), "9");
        assert_eq!(d.star2(INPUT), "34");
    }
}
