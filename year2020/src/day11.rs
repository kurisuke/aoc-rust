use common::day::Day;
use util::grid2d::{Coords, Grid2D, Wrap};

pub struct Day11 {}

static DIRS: &[(i64, i64)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        format!("{}", iterate(input, 4, direct_neighbors))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", iterate(input, 5, visible_neighbors))
    }
}

fn iterate<F>(input: &str, max_occupied: usize, nb_fn: F) -> usize
where
    F: Fn(&Grid2D<char>, Coords) -> Vec<&char>,
{
    let mut grid = Grid2D::new(input).unwrap();

    let mut all_occupied = grid.count('#');
    loop {
        let mut grid_new = grid.clone();
        for (pos, pos_val) in grid.enumerate() {
            if *pos_val != '.' {
                let nb: Vec<_> = nb_fn(&grid, pos);
                let num_occupied = nb.into_iter().filter(|&&v| v == '#').count();
                if *pos_val == 'L' && num_occupied == 0 {
                    grid_new.set(&pos, '#');
                } else if *pos_val == '#' && num_occupied >= max_occupied {
                    grid_new.set(&pos, 'L');
                }
            }
        }

        let all_occupied_new = grid_new.count('#');
        if all_occupied_new != all_occupied {
            all_occupied = all_occupied_new;
            grid = grid_new;
        } else {
            break;
        }
    }

    all_occupied
}

fn direct_neighbors(grid: &Grid2D<char>, coords: Coords) -> Vec<&char> {
    grid.neighbors(&coords)
        .into_iter()
        .filter_map(|v| v)
        .collect()
}

fn visible_neighbors(grid: &Grid2D<char>, coords: Coords) -> Vec<&char> {
    DIRS.iter()
        .map(|d| {
            grid.traverse_init_wrap(&coords, &Coords { x: d.0, y: d.1 }, Wrap::None)
                .skip(1)
                .find(|&&v| v != '.')
        })
        .filter_map(|v| v)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let d = Day11 {};
        assert_eq!(d.star1(input), "37");
        assert_eq!(d.star2(input), "26");
    }
}
