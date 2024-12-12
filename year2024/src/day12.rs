use std::collections::VecDeque;

use common::day::Day;
use util::grid2d::Grid2D;

pub struct Day12 {}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let regions = find_regions(&grid);
        let price = regions
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum::<usize>();
        price.to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn find_regions(grid: &Grid2D<char>) -> Vec<(usize, usize)> {
    let mut visited = Grid2D::with_default(grid.dimensions(), &false);
    let mut regions = vec![];

    while let Some(unused) = visited.find(false) {
        let region_char = grid.at(&unused).unwrap();
        let mut region_area = 0;
        let mut region_perimeter = 0;

        let mut frontier = VecDeque::new();
        visited.set(&unused, true);
        frontier.push_back(unused);
        region_area += 1;
        region_perimeter += grid
            .neighbors_cardinal(&unused)
            .into_iter()
            .filter(|c| c.is_none() || c.unwrap() != region_char)
            .count();

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
                    }
                }
            }
        }

        // println!("char: {region_char}, pos: {unused}, area: {region_area}, perimeter: {region_perimeter}");
        regions.push((region_area, region_perimeter));
    }

    regions
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
}
