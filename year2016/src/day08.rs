use common::day::Day;
use std::collections::VecDeque;
use util::grid2d::{Coords, Grid2D};

pub struct Day08 {}

enum Cmd {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn apply_cmd(display: &mut Grid2D<char>, cmd: &Cmd) {
    match cmd {
        Cmd::Rect(x_max, y_max) => {
            for x in 0..(*x_max as usize) {
                for y in 0..(*y_max as usize) {
                    display.set(
                        &Coords {
                            x: x as i64,
                            y: y as i64,
                        },
                        '█',
                    );
                }
            }
        }
        Cmd::RotateRow(y, k) => {
            let mut row: VecDeque<_> = display
                .row(*y as i64)
                .unwrap()
                .into_iter()
                .copied()
                .collect();
            row.rotate_right(*k % display.width() as usize);
            let row: Vec<_> = row.into_iter().collect();
            display.set_row(*y as i64, row);
        }
        Cmd::RotateCol(x, k) => {
            let mut col: VecDeque<_> = display
                .col(*x as i64)
                .unwrap()
                .into_iter()
                .copied()
                .collect();
            col.rotate_right(*k % display.height() as usize);
            let col: Vec<_> = col.into_iter().collect();
            display.set_col(*x as i64, col);
        }
    }
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .filter_map(|line| {
            if line.starts_with("rect") {
                let parts: Vec<_> = line.split_whitespace().collect();
                let coords: Vec<_> = parts[1]
                    .split('x')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                Some(Cmd::Rect(coords[0], coords[1]))
            } else if line.starts_with("rotate row") {
                let parts: Vec<_> = line.split('=').collect();
                let params: Vec<_> = parts[1].split(" by ").collect();
                let y = params[0].parse::<usize>().unwrap();
                let k = params[1].parse::<usize>().unwrap();
                Some(Cmd::RotateRow(y, k))
            } else if line.starts_with("rotate column") {
                let parts: Vec<_> = line.split('=').collect();
                let params: Vec<_> = parts[1].split(" by ").collect();
                let x = params[0].parse::<usize>().unwrap();
                let k = params[1].parse::<usize>().unwrap();
                Some(Cmd::RotateCol(x, k))
            } else {
                None
            }
        })
        .collect()
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut display = Grid2D::with_default(Coords { x: 50, y: 6 }, &' ');
        for cmd in cmds {
            apply_cmd(&mut display, &cmd);
        }
        format!("{}", display.count('█'))
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut display = Grid2D::with_default(Coords { x: 50, y: 6 }, &' ');
        for cmd in cmds {
            apply_cmd(&mut display, &cmd);
        }
        format!("{}", display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1"#;
        let cmds = parse_input(input);
        let mut display = Grid2D::with_default(Coords { x: 7, y: 3 }, &' ');
        for cmd in cmds {
            apply_cmd(&mut display, &cmd);
        }
        assert_eq!(display.count('█'), 6);
        let expected = r#" █  █ █
█ █    
 █     "#;
        assert_eq!(format!("{}", display), expected);
    }
}
