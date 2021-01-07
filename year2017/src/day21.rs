use common::day::Day;
use std::collections::HashMap;
use util::grid2d::{Coords, Flip, Grid2D};

pub struct Day21 {}

type Replacements = HashMap<Grid2D<char>, Grid2D<char>>;

fn cut(grid: &Grid2D<char>, cut_size: i64) -> Vec<Grid2D<char>> {
    let mut sub_grids = vec![];
    for y in (0..grid.height()).step_by(cut_size as usize) {
        for x in (0..grid.width()).step_by(cut_size as usize) {
            let c1 = Coords { x, y };
            let c2 = Coords {
                x: x + cut_size,
                y: y + cut_size,
            };
            sub_grids.push(grid.clip(c1, c2).unwrap());
        }
    }
    sub_grids
}

fn replace(tiles: &[Grid2D<char>], replacements: &Replacements) -> Vec<Grid2D<char>> {
    tiles
        .iter()
        .map(|tile| replacements.get(&tile).unwrap().clone())
        .collect()
}

fn merge(tiles: Vec<Grid2D<char>>) -> Grid2D<char> {
    let size = (tiles.len() as f64).sqrt() as i64;
    let tile_width = tiles[0].width();
    let new_grid_size = size * tile_width;
    let mut new_grid = Grid2D::with_default(
        Coords {
            x: new_grid_size,
            y: new_grid_size,
        },
        &'.',
    );
    let mut i = 0;
    for y in (0..new_grid_size).step_by(tile_width as usize) {
        for x in (0..new_grid_size).step_by(tile_width as usize) {
            new_grid.paste(Coords { x, y }, &tiles[i]);
            i += 1;
        }
    }
    new_grid
}

fn evolve(grid: Grid2D<char>, replacements: &Replacements) -> Grid2D<char> {
    let cut_size = if grid.width() % 2 == 0 {
        2
    } else if grid.width() % 3 == 0 {
        3
    } else {
        panic!("Cannot evolve with grid size: {}", grid.width());
    };

    let tiles = cut(&grid, cut_size);
    let replaced_tiles = replace(&tiles, &replacements);
    merge(replaced_tiles)
}

fn parse_input(input: &str) -> Replacements {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" => ").collect();
            let key = Grid2D::new(&parts[0].replace("/", "\n")).unwrap();
            let value = Grid2D::new(&parts[1].replace("/", "\n")).unwrap();
            let mut transforms = vec![];
            for flip in [Flip::FlipNone, Flip::FlipH].iter() {
                let trans = key.flip(*flip);
                transforms.push((trans.clone(), value.clone()));
                transforms.push((trans.rotate90(), value.clone()));
                transforms.push((trans.rotate90().rotate90(), value.clone()));
                transforms.push((trans.rotate90().rotate90().rotate90(), value.clone()));
            }
            transforms
        })
        .flatten()
        .collect()
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let replacements = parse_input(input);
        let mut grid = Grid2D::new(
            r#".#.
..#
###"#,
        )
        .unwrap();
        for _ in 0..5 {
            grid = evolve(grid, &replacements);
        }
        format!("{}", grid.count('#'))
    }

    fn star2(&self, input: &str) -> String {
        let replacements = parse_input(input);
        let mut grid = Grid2D::new(
            r#".#.
..#
###"#,
        )
        .unwrap();
        for _ in 0..18 {
            grid = evolve(grid, &replacements);
        }
        format!("{}", grid.count('#'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"#;
        let replacements = parse_input(input);
        let mut grid = Grid2D::new(
            r#".#.
..#
###"#,
        )
        .unwrap();
        for _ in 0..2 {
            grid = evolve(grid, &replacements);
        }
        assert_eq!(grid.count('#'), 12);
    }
}
