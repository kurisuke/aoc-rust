use common::day::Day;
use md5::{Digest, Md5};
use std::collections::VecDeque;

pub struct Day17 {}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    steps: String,
}

fn neighbors(current: &State, salt: &str) -> Vec<State> {
    let mut hasher = Md5::new();
    hasher.update(salt.as_bytes());
    hasher.update(current.steps.as_bytes());
    let hash = hasher.finalize_reset();
    let open_up = (hash[0] >> 4) >= 0xb;
    let open_down = (hash[0] & 0x0f) >= 0xb;
    let open_left = (hash[1] >> 4) >= 0xb;
    let open_right = (hash[1] & 0x0f) >= 0xb;

    let mut neighbors = vec![];
    if current.pos.1 > 0 && open_up {
        neighbors.push(State {
            pos: (current.pos.0, current.pos.1 - 1),
            steps: format!("{}U", current.steps),
        });
    }
    if current.pos.1 < 3 && open_down {
        neighbors.push(State {
            pos: (current.pos.0, current.pos.1 + 1),
            steps: format!("{}D", current.steps),
        });
    }
    if current.pos.0 > 0 && open_left {
        neighbors.push(State {
            pos: (current.pos.0 - 1, current.pos.1),
            steps: format!("{}L", current.steps),
        });
    }
    if current.pos.0 < 3 && open_right {
        neighbors.push(State {
            pos: (current.pos.0 + 1, current.pos.1),
            steps: format!("{}R", current.steps),
        });
    }
    neighbors
}

fn search(salt: &str, early_exit: bool) -> Option<String> {
    let mut frontier = VecDeque::new();
    let mut solutions = vec![];
    let init_pos = State {
        pos: (0, 0),
        steps: String::new(),
    };
    frontier.push_back(init_pos);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        if current.pos == (3, 3) {
            if early_exit {
                return Some(current.steps);
            } else {
                solutions.push(current.steps);
            }
        } else {
            for next in neighbors(&current, salt) {
                frontier.push_back(next);
            }
        }
    }
    solutions.into_iter().max_by(|a, b| a.len().cmp(&b.len()))
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        search(input.trim(), true).unwrap()
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", search(input.trim(), false).unwrap().len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day17 {};
        assert_eq!(d.star1("ihgpwlah"), "DDRRRD");
        assert_eq!(d.star1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(d.star1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn star2() {
        let d = Day17 {};
        assert_eq!(d.star2("ihgpwlah"), "370");
        assert_eq!(d.star2("kglvqrro"), "492");
        assert_eq!(d.star2("ulqzkmiv"), "830");
    }
}
