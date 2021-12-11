use common::day::Day;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Day13 {}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: usize,
    target_dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.target_dist)
            .cmp(&(self.cost + self.target_dist))
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dist(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn is_wall(pos: &(usize, usize), magic: usize) -> bool {
    let x = pos.0;
    let y = pos.1;
    let prod = x * x + 3 * x + 2 * x * y + y + y * y;
    let sum = prod + magic;
    let binary_repr = format!("{:b}", sum);
    binary_repr.chars().filter(|c| c == &'1').count() % 2 == 1
}

fn neighbors(pos: &(usize, usize), magic: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    // LEFT
    if pos.0 > 0 && !is_wall(&(pos.0 - 1, pos.1), magic) {
        ret.push((pos.0 - 1, pos.1));
    }
    // UP
    if pos.1 > 0 && !is_wall(&(pos.0, pos.1 - 1), magic) {
        ret.push((pos.0, pos.1 - 1));
    }
    // RIGHT
    if !is_wall(&(pos.0 + 1, pos.1), magic) {
        ret.push((pos.0 + 1, pos.1));
    }
    // BOTTOM
    if !is_wall(&(pos.0, pos.1 + 1), magic) {
        ret.push((pos.0, pos.1 + 1));
    }
    ret
}

fn fill(init_pos: (usize, usize), max_steps: usize, magic: usize) -> usize {
    let mut reached = HashSet::new();
    let mut frontier = HashSet::new();
    frontier.insert(init_pos);
    for _ in 0..=max_steps {
        let mut new_frontier = HashSet::new();
        for current in frontier.iter() {
            for next in neighbors(current, magic) {
                if !frontier.contains(&next) && !reached.contains(&next) {
                    new_frontier.insert(next);
                }
            }
        }
        reached.extend(frontier);
        frontier = new_frontier;
    }
    reached.len()
}

fn search(init_pos: (usize, usize), target_pos: (usize, usize), magic: usize) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    let start = State {
        position: init_pos,
        cost: 0,
        target_dist: dist(&init_pos, &target_pos),
    };
    frontier.push(start);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(init_pos, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.position == target_pos {
            return Some(current.cost);
        }

        for next in neighbors(&current.position, magic) {
            let new_cost = cost_so_far.get(&current.position).unwrap() + 1;
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                let e = cost_so_far.entry(next).or_insert(0);
                *e = new_cost;
                let target_dist = new_cost + dist(&next, &target_pos);
                frontier.push(State {
                    position: next,
                    cost: new_cost,
                    target_dist,
                });
            }
        }
    }
    None
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let magic = input.parse::<usize>().unwrap();
        let steps = search((1, 1), (31, 39), magic).unwrap();
        format!("{}", steps)
    }

    fn star2(&self, input: &str) -> String {
        let magic = input.parse::<usize>().unwrap();
        let filled = fill((1, 1), 50, magic);
        format!("{}", filled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(search((1, 1), (7, 4), 10), Some(11));
    }
}
