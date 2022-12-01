use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day25 {}

fn parse_input(input: &str) -> Grid2D<char> {
    Grid2D::new(input).unwrap()
}

fn move_cucumbers(grid: &mut Grid2D<char>, cucumber: char, offset: &Coords) {
    let move_coords: Vec<_> = grid
        .coords_iter()
        .filter(|c| grid.at(c).unwrap() == &cucumber)
        .filter_map(|c| {
            let t = Coords {
                x: (c.x + offset.x) % grid.width(),
                y: (c.y + offset.y) % grid.height(),
            };
            if grid.at(&t).unwrap() == &'.' {
                Some((c, t))
            } else {
                None
            }
        })
        .collect();
    for (c, t) in move_coords {
        grid.set(&c, '.');
        grid.set(&t, cucumber);
    }
}

fn run(mut grid: Grid2D<char>) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        let old_grid = grid.clone();
        // move east
        move_cucumbers(&mut grid, '>', &Coords { x: 1, y: 0 });
        // move south
        move_cucumbers(&mut grid, 'v', &Coords { x: 0, y: 1 });
        if grid == old_grid {
            break;
        }
    }
    i
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        format!("{}", run(grid))
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
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "58");
    }
}
