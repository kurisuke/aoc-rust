use crate::day::Day;
use crate::grid2d::{Grid2D, Wrap};

pub struct Day11 {}

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
    F: Fn(&Grid2D, i64, i64) -> Vec<char>,
{
    let mut grid = Grid2D::new(input).unwrap();

    let mut all_occupied = grid.count('#');
    loop {
        let mut grid_new = grid.clone();
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let pos_val = grid.at(x, y).unwrap();
                if pos_val != '.' {
                    let nb: Vec<_> = nb_fn(&grid, x, y);
                    let num_occupied = nb.iter().filter(|&&v| v == '#').count();
                    if pos_val == 'L' && num_occupied == 0 {
                        grid_new.set(x, y, '#');
                    } else if pos_val == '#' && num_occupied >= max_occupied {
                        grid_new.set(x, y, 'L');
                    }
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

fn direct_neighbors(grid: &Grid2D, x: i64, y: i64) -> Vec<char> {
    grid.neighbors(x, y).into_iter().filter_map(|v| v).collect()
}

fn visible_neighbors(grid: &Grid2D, x: i64, y: i64) -> Vec<char> {
    let dirs = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    dirs.iter()
        .map(|d| {
            grid.traverse_init_wrap(x + d.0, y + d.1, d.0, d.1, Wrap::None)
                .find(|&v| v == 'L' || v == '#')
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
