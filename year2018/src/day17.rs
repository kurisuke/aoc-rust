use common::day::Day;
use regex::Regex;
use util::grid2d::{Coords, Direction, Grid2D};

struct ScanLine {
    axis_fix: char,
    start: Coords,
    end: Coords,
}

fn parse_input(input: &str) -> Grid2D<char> {
    let re_line = Regex::new(r"([xy])=(\d+), \w=(\d+)..(\d+)").unwrap();

    let scan_lines: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = re_line.captures(line).unwrap();
            let axis_fix = caps.get(1).unwrap().as_str().chars().next().unwrap();
            let fix = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let var1 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let var2 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let start = match axis_fix {
                'x' => Coords { x: fix, y: var1 },
                'y' => Coords { x: var1, y: fix },
                _ => {
                    panic!("unexpected axis char: {}", axis_fix);
                }
            };
            let end = match axis_fix {
                'x' => Coords { x: fix, y: var2 },
                'y' => Coords { x: var2, y: fix },
                _ => {
                    panic!("unexpected axis char: {}", axis_fix);
                }
            };
            ScanLine {
                axis_fix,
                start,
                end,
            }
        })
        .collect();

    let x_min = scan_lines
        .iter()
        .map(|sl| sl.start.x.min(sl.end.x))
        .min()
        .unwrap()
        - 1;
    let x_max = scan_lines
        .iter()
        .map(|sl| sl.start.x.max(sl.end.x))
        .max()
        .unwrap()
        + 1;
    let y_max = scan_lines
        .iter()
        .map(|sl| sl.start.y.max(sl.end.y))
        .max()
        .unwrap();

    let mut grid = Grid2D::with_default(
        Coords {
            x: x_max - x_min + 1,
            y: y_max + 1,
        },
        &'.',
    );

    let spring = Coords {
        x: 500 - x_min,
        y: 0,
    };
    grid.set(&spring, '+');

    for scan_line in scan_lines {
        match scan_line.axis_fix {
            'x' => {
                let x = scan_line.start.x;
                for y in scan_line.start.y..=scan_line.end.y {
                    grid.set(&Coords { x: x - x_min, y }, '#');
                }
            }
            'y' => {
                let y = scan_line.start.y;
                for x in scan_line.start.x..=scan_line.end.x {
                    grid.set(&Coords { x: x - x_min, y }, '#');
                }
            }
            _ => {
                panic!("unexpected axis char: {}", scan_line.axis_fix);
            }
        }
    }

    grid
}

fn flow(mut grid: Grid2D<char>) -> Grid2D<char> {
    let mut heads = vec![grid.find('+').unwrap()];
    // println!("Flow starts at: {}", heads[0]);

    while let Some(head) = heads.pop() {
        // stop if bottom reached
        if head.y >= grid.height() - 1_i64 {
            continue;
        }

        // check down
        let down = head.mov(Direction::S);
        match grid.at(&down).unwrap() {
            '~' | '#' => {
                let new_flows = flow_lr(&mut grid, &head);
                heads.extend(new_flows);
            }
            _ => {
                grid.set(&down, '|');
                heads.push(down);
            }
        }

        // println!("{}\n", grid);
    }

    grid
}

fn flow_lr(grid: &mut Grid2D<char>, head: &Coords) -> Vec<Coords> {
    let mut new_flows = vec![];

    // find left
    let mut left = *head;
    let mut left_fill_border = None;
    loop {
        left = left.mov(Direction::W);
        let left_ch = grid.at(&left).unwrap();

        if matches!(left_ch, '~' | '#') {
            left_fill_border = Some(left);
            break;
        } else {
            grid.set(&left, '|');
            let left_down = left.mov(Direction::S);
            let left_down_ch = grid.at(&left_down).unwrap();
            if !matches!(left_down_ch, '~' | '#') {
                new_flows.push(left);
                break;
            }
        }
    }

    // find right
    let mut right = *head;
    loop {
        right = right.mov(Direction::E);
        let right_ch = grid.at(&right).unwrap();

        if matches!(right_ch, '~' | '#') {
            if let Some(fill_pos) = left_fill_border {
                // fill the line
                let mut fill_pos = fill_pos.mov(Direction::E);
                while fill_pos != right {
                    grid.set(&fill_pos, '~');
                    fill_pos = fill_pos.mov(Direction::E);
                }

                // new_flow is head(x, y-1)
                let up = head.mov(Direction::N);
                new_flows.push(up);
            }
            break;
        } else {
            grid.set(&right, '|');
            let right_down = right.mov(Direction::S);
            let right_down_ch = grid.at(&right_down).unwrap();
            if !matches!(right_down_ch, '~' | '#') {
                new_flows.push(right);
                break;
            }
        }
    }

    new_flows
}

pub struct Day17 {}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let grid_after = flow(grid);
        let min_y_clay = grid_after.find('#').unwrap().y;
        let water_tiles = grid_after.count('|') + grid_after.count('~') - min_y_clay as usize + 1;
        format!("{}", water_tiles)
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);
        let grid_after = flow(grid);
        let retain_tiles = grid_after.count('~');
        format!("{}", retain_tiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day17 {};
        let input = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;
        assert_eq!(d.star1(input), "57");
        assert_eq!(d.star2(input), "29");
    }
}
