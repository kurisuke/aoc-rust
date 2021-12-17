use common::day::Day;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use util::grid2d::Coords;

pub struct Day22 {}

enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Tool {
    None,
    Torch,
    Gear,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PosTool {
    pos: Coords,
    tool: Tool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pt: PosTool,
    cost: u64,
    target_dist: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.target_dist)
            .cmp(&(self.cost + self.target_dist))
            .then_with(|| other.pt.pos.cmp(&self.pt.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn erosion(c: &Coords, depth: usize, target: &Coords, cache: &mut HashMap<Coords, usize>) -> usize {
    if c.x == 0 && c.y == 0 || c == target {
        depth
    } else if c.y == 0 {
        (c.x as usize * 16807 + depth) % 20183
    } else if c.x == 0 {
        (c.y as usize * 48271 + depth) % 20183
    } else if let Some(e) = cache.get(c) {
        *e
    } else {
        let e = (erosion(&Coords { x: c.x - 1, y: c.y }, depth, target, cache)
            * erosion(&Coords { x: c.x, y: c.y - 1 }, depth, target, cache)
            + depth)
            % 20183;
        cache.insert(*c, e);
        e
    }
}

fn terrain(
    c: &Coords,
    depth: usize,
    target: &Coords,
    cache: &mut HashMap<Coords, usize>,
) -> Terrain {
    match erosion(c, depth, target, cache) % 3 {
        0 => Terrain::Rocky,
        1 => Terrain::Wet,
        2 => Terrain::Narrow,
        _ => unreachable!(),
    }
}

fn total_risk(depth: usize, target: &Coords) -> usize {
    let mut cache = HashMap::new();
    let mut sum_risk = 0;
    for x in 0..=target.x {
        for y in 0..=target.y {
            let e = erosion(&Coords { x, y }, depth, target, &mut cache);
            sum_risk += e % 3;
        }
    }
    sum_risk
}

fn search(depth: usize, target_pos: &Coords) -> Option<u64> {
    let mut cache = HashMap::new();
    let mut frontier = BinaryHeap::new();

    let init_pos = Coords { x: 0, y: 0 };
    let target_pt = PosTool {
        pos: *target_pos,
        tool: Tool::Torch,
    };

    let start = State {
        pt: PosTool {
            pos: init_pos,
            tool: Tool::Torch,
        },
        cost: 0,
        target_dist: init_pos.manhattan(target_pos),
    };
    frontier.push(start);

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start.pt, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.pt == target_pt {
            return Some(current.cost);
        }

        for (next, next_cost) in neighbors(&current, depth, target_pos, &mut cache) {
            let new_cost = cost_so_far.get(&current.pt).unwrap() + next_cost;
            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                let target_dist = new_cost + next.pos.manhattan(&target_pt.pos);
                frontier.push(State {
                    pt: next,
                    cost: new_cost,
                    target_dist,
                });
            }
        }
    }

    None
}

fn neighbors(
    current: &State,
    depth: usize,
    target: &Coords,
    cache: &mut HashMap<Coords, usize>,
) -> Vec<(PosTool, u64)> {
    let mut ns = vec![];

    // change position
    let neighbor_coords = vec![
        Coords {
            x: current.pt.pos.x - 1,
            y: current.pt.pos.y,
        },
        Coords {
            x: current.pt.pos.x + 1,
            y: current.pt.pos.y,
        },
        Coords {
            x: current.pt.pos.x,
            y: current.pt.pos.y - 1,
        },
        Coords {
            x: current.pt.pos.x,
            y: current.pt.pos.y + 1,
        },
    ];

    for nc in neighbor_coords {
        if nc.x >= 0 && nc.y >= 0 {
            let can_move = match terrain(&nc, depth, target, cache) {
                Terrain::Rocky => current.pt.tool != Tool::None,
                Terrain::Wet => current.pt.tool != Tool::Torch,
                Terrain::Narrow => current.pt.tool != Tool::Gear,
            };
            if can_move {
                ns.push((
                    PosTool {
                        pos: nc,
                        tool: current.pt.tool,
                    },
                    1,
                ));
            }
        }
    }

    // change tool
    let tools = vec![Tool::None, Tool::Torch, Tool::Gear];

    let current_terrain = terrain(&current.pt.pos, depth, target, cache);
    for t in tools {
        if t != current.pt.tool {
            let can_use = match current_terrain {
                Terrain::Rocky => t != Tool::None,
                Terrain::Wet => t != Tool::Torch,
                Terrain::Narrow => t != Tool::Gear,
            };
            if can_use {
                ns.push((
                    PosTool {
                        pos: current.pt.pos,
                        tool: t,
                    },
                    7,
                ));
            }
        }
    }

    ns
}

fn parse_input(input: &str) -> (usize, Coords) {
    let dl = input.lines().next().unwrap();
    let depth = dl.split_once(": ").unwrap().1.parse().unwrap();

    let cl = input.lines().nth(1).unwrap();
    let coord_str = cl.split_once(": ").unwrap().1;
    let cv: Vec<_> = coord_str.split(',').collect();
    (
        depth,
        Coords {
            x: cv[0].parse().unwrap(),
            y: cv[1].parse().unwrap(),
        },
    )
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let (depth, target) = parse_input(input);
        format!("{}", total_risk(depth, &target))
    }

    fn star2(&self, input: &str) -> String {
        let (depth, target) = parse_input(input);
        format!("{}", search(depth, &target).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"depth: 510
target: 10,10"#;

        let d = Day22 {};
        assert_eq!(d.star1(input), "114");
        assert_eq!(d.star2(input), "45");
    }
}
