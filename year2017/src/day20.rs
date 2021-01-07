use common::day::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

pub struct Day20 {}

struct Particle {
    pos: Coords3D,
    vel: Coords3D,
    acc: Coords3D,
}

impl Particle {
    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Coords3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for Coords3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Coords3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Coords3D {
    fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

fn parse_coords(s: &str) -> Coords3D {
    let v: Vec<_> = s.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    Coords3D {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

fn parse_input(input: &str) -> Vec<Particle> {
    let re = Regex::new(r"p=<([-,\d]+)>, v=<([-,\d]+)>, a=<([-,\d]+)>").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let pos = parse_coords(caps.get(1).unwrap().as_str());
            let vel = parse_coords(caps.get(2).unwrap().as_str());
            let acc = parse_coords(caps.get(3).unwrap().as_str());
            Particle { pos, vel, acc }
        })
        .collect()
}

fn tick(particles: &mut HashMap<usize, Particle>) {
    // update all
    for v in particles.values_mut() {
        v.update();
    }

    remove_collisions(particles);
}

fn remove_collisions(particles: &mut HashMap<usize, Particle>) {
    let mut to_remove = HashSet::new();
    for c in particles.keys().combinations(2) {
        if particles.get(&c[0]).unwrap().pos == particles.get(&c[1]).unwrap().pos {
            to_remove.insert(*c[0]);
            to_remove.insert(*c[1]);
        }
    }

    for r in to_remove.iter() {
        particles.remove(r);
    }
}

fn simulate(mut particles: HashMap<usize, Particle>, min_ticks_no_collision: usize) -> usize {
    let mut ticks_no_collision = 0;
    let mut last_len = particles.len();

    while last_len > 1 && ticks_no_collision < min_ticks_no_collision {
        tick(&mut particles);

        let new_len = particles.len();
        if new_len < last_len {
            ticks_no_collision = 0;
            last_len = new_len;
        } else {
            ticks_no_collision += 1;
        }
    }

    particles.len()
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let particles = parse_input(input);
        let closest = particles
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.acc.abs().cmp(&b.1.acc.abs()))
            .unwrap()
            .0;
        format!("{}", closest)
    }

    fn star2(&self, input: &str) -> String {
        let particles = parse_input(input);
        let particles_map: HashMap<_, _> = particles.into_iter().enumerate().collect();
        format!("{}", simulate(particles_map, 20))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day20 {};
        let input = r#"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"#;
        assert_eq!(d.star1(input), "0");
    }

    #[test]
    fn ex2() {
        let d = Day20 {};
        let input = r#"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"#;
        assert_eq!(d.star2(input), "1");
    }
}
