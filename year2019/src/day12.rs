use common::day::Day;
use scan_fmt::scan_fmt;
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
struct System {
    state: [Body; 4],
}

impl System {
    fn update(&mut self) {
        self.update_vel();
        self.update_pos();
    }

    fn update_vel(&mut self) {
        for i in 0..4 {
            for j in (i + 1)..4 {
                for c in 0..3 {
                    match self.state[i].pos[c].cmp(&self.state[j].pos[c]) {
                        Ordering::Less => {
                            self.state[i].vel[c] += 1;
                            self.state[j].vel[c] -= 1;
                        }
                        Ordering::Greater => {
                            self.state[i].vel[c] -= 1;
                            self.state[j].vel[c] += 1;
                        }
                        Ordering::Equal => {}
                    }
                }
            }
        }
    }

    fn update_pos(&mut self) {
        for i in 0..4 {
            for c in 0..3 {
                self.state[i].pos[c] += self.state[i].vel[c];
            }
        }
    }

    fn total_energy(&self) -> i64 {
        let mut e = 0;
        for i in 0..4 {
            let pot = (0..3).map(|c| self.state[i].pos[c].abs()).sum::<i64>();
            let kin = (0..3).map(|c| self.state[i].vel[c].abs()).sum::<i64>();
            e += pot * kin;
        }
        e
    }

    fn state_axis(&self, c: usize) -> i64 {
        (self.state[0].pos[c] << 56)
            ^ (self.state[0].vel[c] << 48)
            ^ (self.state[1].pos[c] << 40)
            ^ (self.state[1].vel[c] << 32)
            ^ (self.state[2].pos[c] << 24)
            ^ (self.state[2].vel[c] << 16)
            ^ (self.state[3].pos[c] << 8)
            ^ self.state[3].vel[c]
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Body {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn parse_input(input: &str) -> System {
    let moons: Vec<_> = input
        .lines()
        .map(|l| {
            let (pos_x, pos_y, pos_z) =
                scan_fmt!(l, "<x={d}, y={d}, z={d}>", i64, i64, i64).unwrap();
            Body {
                pos: [pos_x, pos_y, pos_z],
                vel: [0, 0, 0],
            }
        })
        .collect();
    System {
        state: [moons[0], moons[1], moons[2], moons[3]],
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

pub struct Day12 {}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let mut system = parse_input(input);
        for _ in 0..1000 {
            system.update();
        }
        format!("{}", system.total_energy())
    }

    fn star2(&self, input: &str) -> String {
        let mut system = parse_input(input);
        let states_init = [
            system.state_axis(0),
            system.state_axis(1),
            system.state_axis(2),
        ];
        let mut periods = [None, None, None];
        let mut steps = 0;
        while periods.iter().any(|p| p.is_none()) {
            steps += 1;
            system.update();
            let states = [
                system.state_axis(0),
                system.state_axis(1),
                system.state_axis(2),
            ];
            for c in 0..3 {
                if periods[c].is_none() && states[c] == states_init[c] {
                    periods[c] = Some(steps);
                }
            }
        }
        let period_total = periods
            .iter()
            .flatten()
            .copied()
            .reduce(|a, i| a * i / gcd(a, i))
            .unwrap();
        format!("{}", period_total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#;
        let mut system = parse_input(input);
        for _ in 0..100 {
            system.update();
        }
        assert_eq!(system.total_energy(), 1940);
    }
}
