use common::day::Day;

use std::cmp::Ordering;
use std::collections::HashSet;

use util::grid2d::{Coords, Grid2D};

pub struct Day13 {}

enum FoldInstr {
    X(i64),
    Y(i64),
}

fn parse_input(input: &str) -> (HashSet<Coords>, Vec<FoldInstr>) {
    let dots_str = input.split("\n\n").next().unwrap();
    let dots: HashSet<_> = dots_str
        .lines()
        .map(|l| {
            let spl: Vec<_> = l.split(',').collect();
            Coords {
                x: spl[0].parse::<i64>().unwrap(),
                y: spl[1].parse::<i64>().unwrap(),
            }
        })
        .collect();

    let folds_str = input.split("\n\n").nth(1).unwrap();
    let folds: Vec<_> = folds_str
        .lines()
        .map(|l| {
            let spl: Vec<_> = l.split(' ').collect();
            let spl_eq: Vec<_> = spl[2].split('=').collect();
            let v = spl_eq[1].parse::<i64>().unwrap();
            match spl_eq[0] {
                "x" => FoldInstr::X(v),
                "y" => FoldInstr::Y(v),
                _ => panic!("cannot parse fold instruction: {}", spl[2]),
            }
        })
        .collect();

    (dots, folds)
}

fn fold(dots_in: &HashSet<Coords>, instr: &FoldInstr) -> HashSet<Coords> {
    dots_in
        .iter()
        .filter_map(|c| match instr {
            FoldInstr::X(fold_x) => match c.x.cmp(fold_x) {
                Ordering::Less => Some(Coords { x: c.x, y: c.y }),
                Ordering::Equal => None,
                Ordering::Greater => {
                    let new_x = -c.x + 2 * fold_x;
                    Some(Coords { x: new_x, y: c.y })
                }
            },
            FoldInstr::Y(fold_y) => match c.y.cmp(fold_y) {
                Ordering::Less => Some(Coords { x: c.x, y: c.y }),
                Ordering::Equal => None,
                Ordering::Greater => {
                    let new_y = -c.y + 2 * fold_y;
                    Some(Coords { x: c.x, y: new_y })
                }
            },
        })
        .collect()
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let (dots, folds) = parse_input(input);
        let dots = fold(&dots, &folds[0]);
        format!("{}", dots.len())
    }

    fn star2(&self, input: &str) -> String {
        let (mut dots, folds) = parse_input(input);
        for f in folds.iter() {
            dots = fold(&dots, f);
        }
        let max_x = dots.iter().map(|c| c.x).max().unwrap();
        let max_y = dots.iter().map(|c| c.y).max().unwrap();

        let mut grid = Grid2D::with_default(
            Coords {
                x: max_x + 1,
                y: max_y + 1,
            },
            &' ',
        );
        for d in dots.iter() {
            grid.set(d, '█');
        }
        format!("{}", grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

        let d = Day13 {};
        assert_eq!(d.star1(input), "17");
    }
}
