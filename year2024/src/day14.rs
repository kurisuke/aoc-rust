use std::collections::HashSet;

use common::day::Day;
use regex::Regex;
use util::grid2d::{Coords, Grid2D};

pub struct Day14 {}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let robots = parse_input(input);
        let limits = Coords { x: 101, y: 103 };
        safety_factor(&robots, &limits, 100).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let robots = parse_input(input);
        let limits = Coords { x: 101, y: 103 };
        let after = smallest_cluster(&robots, &limits);
        // print_pattern(&robots, &limits, after);
        after.to_string()
    }
}

struct Robot {
    p: Coords,
    v: Coords,
}

impl Robot {
    fn pos_after(&self, limits: &Coords, n: i64) -> Coords {
        let n_x = n % limits.x;
        let n_y = n % limits.y;

        let x = (self.p.x + n_x * self.v.x).rem_euclid(limits.x);
        let y = (self.p.y + n_y * self.v.y).rem_euclid(limits.y);

        Coords { x, y }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"^p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot {
                p: Coords {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                v: Coords {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn safety_factor(robots: &[Robot], limits: &Coords, after: i64) -> usize {
    let mut q = [0, 0, 0, 0];

    let mid_x = limits.x / 2;
    let mid_y = limits.y / 2;

    for robot in robots {
        let pos = robot.pos_after(limits, after);
        if pos.x < mid_x && pos.y < mid_y {
            q[0] += 1;
        } else if pos.x > mid_x && pos.y < mid_y {
            q[1] += 1;
        } else if pos.x < mid_x && pos.y > mid_y {
            q[2] += 1;
        } else if pos.x > mid_x && pos.y > mid_y {
            q[3] += 1;
        }
    }

    q[0] * q[1] * q[2] * q[3]
}

fn smallest_cluster(robots: &[Robot], limits: &Coords) -> i64 {
    let mut x_min = robots.len();
    let mut y_min = robots.len();
    let mut n_x = 0;
    let mut n_y = 0;

    for n in 0..limits.x.max(limits.y) {
        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();

        for robot in robots {
            let pos = robot.pos_after(limits, n);
            x_set.insert(pos.x);
            y_set.insert(pos.y);
        }

        if x_set.len() < x_min {
            n_x = n;
            x_min = x_set.len();
        }
        if y_set.len() < y_min {
            n_y = n;
            y_min = y_set.len();
        }
    }

    let mut n = n_x;
    while n % limits.y != n_y {
        n += limits.x;
    }

    n
}

#[allow(dead_code)]
fn print_pattern(robots: &[Robot], limits: &Coords, after: i64) {
    let mut grid = Grid2D::with_default(*limits, &' ');

    for robot in robots {
        let pos = robot.pos_after(limits, after);
        grid.set(&pos, '*');
    }

    println!("{}", grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn star1() {
        let robots = parse_input(INPUT);
        let limits = Coords { x: 11, y: 7 };
        assert_eq!(safety_factor(&robots, &limits, 100), 12);
    }
}
