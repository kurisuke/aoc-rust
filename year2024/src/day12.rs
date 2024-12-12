use std::collections::VecDeque;

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day12 {}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let regions = find_regions(&grid);
        let price = regions
            .into_iter()
            .map(|(area, perimeter, _)| area * perimeter)
            .sum::<usize>();
        price.to_string()
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let regions = find_regions(&grid);
        let price = regions
            .into_iter()
            .map(|(area, _, corners)| area * corners)
            .sum::<usize>();
        price.to_string()
    }
}

fn find_regions(grid: &Grid2D<char>) -> Vec<(usize, usize, usize)> {
    let mut visited = Grid2D::with_default(grid.dimensions(), &false);
    let mut regions = vec![];

    while let Some(unused) = visited.find(false) {
        let region_char = grid.at(&unused).unwrap();
        let mut region_area = 0;
        let mut region_perimeter = 0;
        let mut region_corners_int = 0;
        let mut region_corners_ext = 0;

        let mut frontier = VecDeque::new();
        visited.set(&unused, true);
        frontier.push_back(unused);
        region_area += 1;
        region_perimeter += grid
            .neighbors_cardinal(&unused)
            .into_iter()
            .filter(|c| c.is_none() || c.unwrap() != region_char)
            .count();
        let c = corners(grid, unused);
        region_corners_int += c.0;
        region_corners_ext += c.1;

        while let Some(pos) = frontier.pop_front() {
            for neighbor_pos in grid.neighbors_cardinal_coords(&pos) {
                if let Some(neighbor_char) = grid.at(&neighbor_pos) {
                    if neighbor_char == region_char && !visited.at(&neighbor_pos).unwrap() {
                        visited.set(&neighbor_pos, true);
                        frontier.push_back(neighbor_pos);
                        region_area += 1;
                        region_perimeter += grid
                            .neighbors_cardinal(&neighbor_pos)
                            .into_iter()
                            .filter(|c| c.is_none() || c.unwrap() != region_char)
                            .count();
                        let c = corners(grid, neighbor_pos);
                        region_corners_int += c.0;
                        region_corners_ext += c.1;
                    }
                }
            }
        }

        let region_corners = region_corners_ext + region_corners_int / 3;
        // println!("char: {region_char}, pos: {unused}, area: {region_area}, perimeter: {region_perimeter}, corners: {region_corners}");
        regions.push((region_area, region_perimeter, region_corners));
    }

    regions
}

fn corners(grid: &Grid2D<char>, pos: Coords) -> (usize, usize) {
    let corner_neighbor_list = [
        (Direction::N, Direction::W, Direction::NW),
        (Direction::N, Direction::E, Direction::NE),
        (Direction::S, Direction::E, Direction::SE),
        (Direction::S, Direction::W, Direction::SW),
    ];

    let mut int_corners = 0;
    let mut ext_corners = 0;

    let v = grid.at(&pos).unwrap();
    for neighbor_dirs in corner_neighbor_list {
        let n1 = grid.at(&pos.mov(neighbor_dirs.0)).unwrap_or(&'#');
        let n2 = grid.at(&pos.mov(neighbor_dirs.1)).unwrap_or(&'#');
        let n3 = grid.at(&pos.mov(neighbor_dirs.2)).unwrap_or(&'#');

        match (v == n1, v == n2, v == n3) {
            (true, true, true) => {}
            (true, true, false) => {
                int_corners += 1;
            }
            (true, false, true) => {
                int_corners += 1;
            }
            (true, false, false) => {}
            (false, true, true) => {
                int_corners += 1;
            }
            (false, true, false) => {}
            (false, false, true) => {
                ext_corners += 1;
            }
            (false, false, false) => {
                ext_corners += 1;
            }
        }
    }

    (int_corners, ext_corners)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const INPUT2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const INPUT3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn star1() {
        let d = Day12 {};
        assert_eq!(d.star1(INPUT1), "140");
        assert_eq!(d.star1(INPUT2), "772");
        assert_eq!(d.star1(INPUT3), "1930");
    }

    const INPUT4: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    const INPUT5: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn star2() {
        let d = Day12 {};
        assert_eq!(d.star2(INPUT1), "80");
        assert_eq!(d.star2(INPUT2), "436");
        assert_eq!(d.star2(INPUT3), "1206");
        assert_eq!(d.star2(INPUT4), "236");
        assert_eq!(d.star2(INPUT5), "368");
    }
}
