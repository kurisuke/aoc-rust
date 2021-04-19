use common::day::Day;
use std::collections::HashMap;
use util::grid2d::Grid2D;

pub struct Day18 {}

fn evolve(grid_old: &Grid2D<char>) -> Grid2D<char> {
    let mut grid_new = grid_old.clone();
    for (pos, val) in grid_old.enumerate() {
        let new_val = match val {
            '.' => {
                if grid_old
                    .neighbors(&pos)
                    .into_iter()
                    .filter(|n| n.is_some() && n.unwrap() == &'|')
                    .count()
                    >= 3
                {
                    '|'
                } else {
                    '.'
                }
            }
            '|' => {
                if grid_old
                    .neighbors(&pos)
                    .into_iter()
                    .filter(|n| n.is_some() && n.unwrap() == &'#')
                    .count()
                    >= 3
                {
                    '#'
                } else {
                    '|'
                }
            }
            '#' => {
                if grid_old
                    .neighbors(&pos)
                    .into_iter()
                    .filter(|n| n.is_some() && n.unwrap() == &'|')
                    .count()
                    >= 1
                    && grid_old
                        .neighbors(&pos)
                        .into_iter()
                        .filter(|n| n.is_some() && n.unwrap() == &'#')
                        .count()
                        >= 1
                {
                    '#'
                } else {
                    '.'
                }
            }
            _ => {
                panic!("Invalid character: {}", val);
            }
        };
        grid_new.set(&pos, new_val);
    }
    grid_new
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        for _ in 0..10 {
            grid = evolve(&grid);
        }
        format!("{}", grid.count('|') * grid.count('#'))
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let mut states = HashMap::new();
        let mut i = 0;
        states.insert(grid.clone(), i);
        loop {
            grid = evolve(&grid);
            i += 1;
            if let Some(loop_start) = states.insert(grid.clone(), i) {
                let loop_len = i - loop_start;
                let target = 1000000000;
                let pos = (target - loop_start) % loop_len + loop_start;
                let grid = states
                    .iter()
                    .find_map(|(key, &val)| if val == pos { Some(key) } else { None })
                    .unwrap();
                return format!("{}", grid.count('|') * grid.count('#'));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day18 {};
        let input = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#;
        assert_eq!(d.star1(input), "1147");
    }
}
