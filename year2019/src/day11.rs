use common::day::Day;
use std::collections::HashMap;
use util::grid2d::{Coords, Direction};
use util::intcode::{IntSize, Intcode, RunState};

pub struct Day11 {}

fn turn(last_dir: &Direction, cmd: IntSize) -> Direction {
    match cmd {
        0 => {
            // turn left
            match last_dir {
                Direction::N => Direction::W,
                Direction::E => Direction::N,
                Direction::S => Direction::E,
                Direction::W => Direction::S,
                _ => unreachable!(),
            }
        }
        1 => {
            // turn right
            match last_dir {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn paint(intcode: &mut Intcode, start_color: IntSize) -> HashMap<Coords, IntSize> {
    let mut grid = HashMap::new();
    let mut pos = Coords { x: 0, y: 0 };
    let mut direction = Direction::N;

    grid.insert(pos, start_color);

    loop {
        intcode.write_inp(*grid.get(&pos).unwrap_or(&0));
        intcode.run();
        if intcode.state == RunState::Halted {
            break;
        }

        // paint color
        grid.insert(pos, intcode.read_outp().unwrap());
        direction = turn(&direction, intcode.read_outp().unwrap());
        pos = pos.mov(direction);
    }
    grid
}

fn print_grid(grid: &HashMap<Coords, IntSize>) -> String {
    let x_min = grid.keys().map(|c| c.x).min().unwrap();
    let y_min = grid.keys().map(|c| c.y).min().unwrap();
    let x_max = grid.keys().map(|c| c.x).max().unwrap();
    let y_max = grid.keys().map(|c| c.y).max().unwrap();

    let mut s = String::new();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let ch = if grid.get(&Coords { x, y }).unwrap_or(&0) == &1 {
                '█'
            } else {
                ' '
            };
            s.push(ch);
        }
        if y != y_max {
            s.push('\n');
        }
    }
    s
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        let grid = paint(&mut intcode, 0);
        format!("{}", grid.len())
    }

    fn star2(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        let grid = paint(&mut intcode, 1);
        print_grid(&grid)
    }
}
