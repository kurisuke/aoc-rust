use common::day::Day;

use util::grid2d::{Coords, Grid2D};

struct Map {
    source: Coords,
    grid: Grid2D<char>,
}

fn parse(input: &str, part2: bool) -> Map {
    let mut x_min = i64::MAX;
    let mut x_max = 500;
    let mut y_min = 0;
    let mut y_max = i64::MIN;

    let mut paths = vec![];

    for line in input.lines() {
        let mut segments = vec![];
        for node in line.split(" -> ") {
            let mut dims = node.split(',');
            let x = dims.next().unwrap().parse().unwrap();
            let y = dims.next().unwrap().parse().unwrap();
            segments.push(Coords { x, y });

            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        paths.push(segments);
    }

    if part2 {
        x_min -= (y_max - y_min) + 2;
        x_max += (y_max - y_min) + 2;
        y_max += 2;
    }

    let origin = Coords { x: x_min, y: y_min };
    let source = Coords { x: 500, y: 0 } - origin;
    
    let grid_size = Coords {
        x: x_max - x_min + 1,
        y: y_max - y_min + 1,
    };
    let mut grid = Grid2D::with_default(grid_size, &'.');

    // draw lines
    for path in paths {
        for w in path.windows(2) {
            let start = w[0];
            let end = w[1];

            if start.x == end.x {
                let x = start.x - origin.x;
                let a = start.y.min(end.y) - origin.y;
                let b = start.y.max(end.y) - origin.y;
                for y in a..=b {
                    grid.set(&Coords { x, y }, '#');
                }
            } else if start.y == end.y {
                let y = start.y - origin.y;
                let a = start.x.min(end.x) - origin.x;
                let b = start.x.max(end.x) - origin.x;
                for x in a..=b {
                    grid.set(&Coords { x, y }, '#');
                }
            } else {
                unreachable!();
            }
        }
    }
    grid.set(&source, '+');

    if part2 {
        let y = grid.height() - 1;
        for x in 0..grid.width() {
            grid.set(&Coords {x, y}, '#');
        }
    }

    Map { source, grid }
}

impl Map {
    fn pour(&mut self) {
        let mut pos = self.source;
        while self.grid.at(&pos).is_some() {
            let next_pos_list = [
                Coords {
                    x: pos.x,
                    y: pos.y + 1,
                },
                Coords {
                    x: pos.x - 1,
                    y: pos.y + 1,
                },
                Coords {
                    x: pos.x + 1,
                    y: pos.y + 1,
                },
            ];

            let mut settled = true;
            for next_pos in next_pos_list {
                let next_tile = self.grid.at(&next_pos).unwrap_or(&'.');
                if next_tile == &'.' {
                    pos = next_pos;
                    settled = false;
                    break;
                }
            }

            if settled {
                self.grid.set(&pos, 'o');
                break;
            }
        }
        // left the grid area, done
    }
}

fn run(mut map: Map) -> usize {
    let mut units_last = 0;
    loop {
        map.pour();
        let units_now = map.grid.count('o');
        if units_now == units_last {
            break;
        } else {
            units_last = units_now;
        }
    }
    units_last
}

pub struct Day14 {}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let map = parse(input, false);
        format!("{}", run(map))
    }

    fn star2(&self, input: &str) -> String {
        let map = parse(input, true);
        format!("{}", run(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let d = Day14 {};
        assert_eq!(d.star1(input), "24");
        assert_eq!(d.star2(input), "93");
    }
}
