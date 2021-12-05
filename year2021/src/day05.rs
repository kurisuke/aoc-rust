use common::day::Day;

use std::collections::HashMap;
use util::grid2d::Coords;

pub struct Day05 {}

type Line = (Coords, Coords);
type OceanFloor = HashMap<Coords, u64>;

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let ps: Vec<_> = l.split(" -> ").collect();
            let p0: Vec<_> = ps[0].split(',').collect();
            let p1: Vec<_> = ps[1].split(',').collect();
            (
                Coords {
                    x: p0[0].parse().unwrap(),
                    y: p0[1].parse().unwrap(),
                },
                Coords {
                    x: p1[0].parse().unwrap(),
                    y: p1[1].parse().unwrap(),
                },
            )
        })
        .collect()
}

fn filter_diag(lines: Vec<Line>) -> Vec<Line> {
    lines
        .into_iter()
        .filter(|l| l.0.x == l.1.x || l.0.y == l.1.y)
        .collect()
}

fn signum(x: i64) -> i64 {
    match x {
        n if n > 0 => 1,
        0 => 0,
        _ => -1,
    }
}

fn draw_lines(lines: Vec<Line>) -> OceanFloor {
    let mut floor = HashMap::new();
    for l in lines {
        let mut pos = l.0;
        let vec = l.1 - l.0;
        let vec = Coords {
            x: signum(vec.x),
            y: signum(vec.y),
        };

        while pos != l.1 {
            let e = floor.entry(pos).or_insert(0);
            *e += 1;
            pos += vec;
        }
        let e = floor.entry(pos).or_insert(0);
        *e += 1;
    }
    floor
}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let lines = parse_input(input);
        let lines = filter_diag(lines);
        let floor = draw_lines(lines);
        let overlap_points = floor.iter().filter(|&(_, v)| *v > 1).count();
        format!("{}", overlap_points)
    }

    fn star2(&self, input: &str) -> String {
        let lines = parse_input(input);
        let floor = draw_lines(lines);
        let overlap_points = floor.iter().filter(|&(_, v)| *v > 1).count();
        format!("{}", overlap_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let d = Day05 {};
        assert_eq!(d.star1(input), "5");
        assert_eq!(d.star2(input), "12");
    }
}
