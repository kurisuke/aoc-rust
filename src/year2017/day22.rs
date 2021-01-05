use crate::day::Day;
use crate::util::grid2d::{Coords, Direction};
use std::collections::HashMap;

pub struct Day22 {}

struct Cursor {
    pos: Coords,
    dir: Direction,
}

enum NodeState {
    Weakened,
    Infected,
    Flagged,
}

impl Cursor {
    fn turn_right(&mut self) {
        match self.dir {
            Direction::N => {
                self.dir = Direction::E;
            }
            Direction::E => {
                self.dir = Direction::S;
            }
            Direction::S => {
                self.dir = Direction::W;
            }
            Direction::W => {
                self.dir = Direction::N;
            }
            _ => {}
        }
    }

    fn turn_left(&mut self) {
        match self.dir {
            Direction::N => {
                self.dir = Direction::W;
            }
            Direction::E => {
                self.dir = Direction::N;
            }
            Direction::S => {
                self.dir = Direction::E;
            }
            Direction::W => {
                self.dir = Direction::S;
            }
            _ => {}
        }
    }

    fn reverse(&mut self) {
        match self.dir {
            Direction::N => {
                self.dir = Direction::S;
            }
            Direction::E => {
                self.dir = Direction::W;
            }
            Direction::S => {
                self.dir = Direction::N;
            }
            Direction::W => {
                self.dir = Direction::E;
            }
            _ => {}
        }
    }
}

#[allow(clippy::map_entry)]
fn burst_star1(node_states: &mut HashMap<Coords, NodeState>, cursor: &mut Cursor) -> bool {
    if node_states.contains_key(&cursor.pos) {
        node_states.remove(&cursor.pos);
        cursor.turn_right();
        cursor.pos = cursor.pos.mov(cursor.dir);
        false
    } else {
        node_states.insert(cursor.pos, NodeState::Infected);
        cursor.turn_left();
        cursor.pos = cursor.pos.mov(cursor.dir);
        true
    }
}

fn burst_star2(node_states: &mut HashMap<Coords, NodeState>, cursor: &mut Cursor) -> bool {
    if let Some(x) = node_states.get(&cursor.pos) {
        match x {
            NodeState::Weakened => {
                node_states
                    .entry(cursor.pos)
                    .and_modify(|e| *e = NodeState::Infected);
                cursor.pos = cursor.pos.mov(cursor.dir);
                true
            }
            NodeState::Infected => {
                node_states
                    .entry(cursor.pos)
                    .and_modify(|e| *e = NodeState::Flagged);
                cursor.turn_right();
                cursor.pos = cursor.pos.mov(cursor.dir);
                false
            }
            NodeState::Flagged => {
                node_states.remove(&cursor.pos);
                cursor.reverse();
                cursor.pos = cursor.pos.mov(cursor.dir);
                false
            }
        }
    } else {
        node_states.insert(cursor.pos, NodeState::Weakened);
        cursor.turn_left();
        cursor.pos = cursor.pos.mov(cursor.dir);
        false
    }
}

fn parse_input(input: &str) -> (HashMap<Coords, NodeState>, Cursor) {
    let mut node_states = HashMap::new();
    let center = (input.lines().next().unwrap().len() / 2) as i64;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                node_states.insert(
                    Coords {
                        x: x as i64,
                        y: y as i64,
                    },
                    NodeState::Infected,
                );
            }
        }
    }
    let cursor = Cursor {
        pos: Coords {
            x: center,
            y: center,
        },
        dir: Direction::N,
    };
    (node_states, cursor)
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let (mut node_states, mut cursor) = parse_input(input);
        let bursts_with_infection = (0..10000)
            .map(|_| burst_star1(&mut node_states, &mut cursor))
            .filter(|&x| x)
            .count();
        format!("{}", bursts_with_infection)
    }

    fn star2(&self, input: &str) -> String {
        let (mut node_states, mut cursor) = parse_input(input);
        let bursts_with_infection = (0..10000000)
            .map(|_| burst_star2(&mut node_states, &mut cursor))
            .filter(|&x| x)
            .count();
        format!("{}", bursts_with_infection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn ex1() {
        let d = Day22 {};
        let input = r#"..#
#..
..."#;
        assert_eq!(d.star1(input), "5587");
        assert_eq!(d.star2(input), "2511944");
    }
}
