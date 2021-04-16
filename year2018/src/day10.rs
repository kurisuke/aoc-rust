use common::day::Day;
use std::collections::HashSet;

type CoordsXy = (isize, isize);

struct Point {
    pub pos: CoordsXy,
    pub vel: CoordsXy,
}

impl Point {
    fn mov(&mut self) {
        self.pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
    }
}

fn positions(points: &[Point]) -> HashSet<CoordsXy> {
    points.iter().map(|x| x.pos).collect()
}

fn is_writing(positions: &HashSet<CoordsXy>) -> bool {
    let min_y = *positions.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *positions.iter().map(|(_, y)| y).max().unwrap();
    max_y - min_y == 9
}

fn print(positions: &HashSet<CoordsXy>) -> String {
    // get top left corner
    let min_x = *positions.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *positions.iter().map(|(_, y)| y).min().unwrap();

    // get bottom right corner
    let max_x = *positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *positions.iter().map(|(_, y)| y).max().unwrap();

    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&(x, y)) {
                s += "#";
            } else {
                s += " ";
            }
        }
        if y != max_y {
            s += "\n"
        }
    }
    s
}

fn iterate(mut points: Vec<Point>) -> (String, usize) {
    let mut iters = 0;
    while !is_writing(&positions(&points)) {
        iters += 1;
        for p in points.iter_mut() {
            p.mov();
        }
    }
    (print(&positions(&points)), iters)
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut points = vec![];
    for line in input.lines() {
        let s: Vec<_> = line.split(&['<', '>'][..]).collect();
        let ps: Vec<_> = s[1].split(',').collect();
        let vs: Vec<_> = s[3].split(',').collect();
        let pos = (
            ps[0].trim().parse::<isize>().unwrap(),
            ps[1].trim().parse::<isize>().unwrap(),
        );
        let vel = (
            vs[0].trim().parse::<isize>().unwrap(),
            vs[1].trim().parse::<isize>().unwrap(),
        );
        points.push(Point { pos, vel });
    }
    points
}

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let points = parse_input(input);
        let (res, _) = iterate(points);
        res
    }

    fn star2(&self, input: &str) -> String {
        let points = parse_input(input);
        let (_, secs) = iterate(points);
        format!("{}", secs)
    }
}
