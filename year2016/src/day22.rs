use common::day::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct Day22 {}

struct Node {
    used: usize,
    avail: usize,
}

type Grid = HashMap<(usize, usize), Node>;

fn parse_input(input: &str) -> Grid {
    let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+\d+T\s+(\d+)T\s+(\d+)T").unwrap();
    input
        .lines()
        .skip(2)
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let used = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let avail = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            ((x, y), Node { used, avail })
        })
        .collect()
}

fn viable_nodes(grid: &Grid) -> usize {
    grid.values()
        .permutations(2)
        .filter(|node| can_move(node[0], node[1]))
        .count()
}

fn can_move(node_a: &Node, node_b: &Node) -> bool {
    node_a.used != 0 && node_a.used <= node_b.avail
}

fn neighbors(pos: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if pos.0 > 0 {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < max.0 {
        ret.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < max.1 {
        ret.push((pos.0, pos.1 + 1));
    }
    ret
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) -> String {
    let mut s = String::new();
    let x_max = grid.keys().map(|pos| pos.0).max().unwrap();
    let y_max = grid.keys().map(|pos| pos.1).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            let node = grid.get(&(x, y)).unwrap();
            if node.used == 0 {
                s.push('_');
            } else if node.used > 94 {
                s.push('█');
            } else if neighbors((x, y), (x_max, y_max))
                .iter()
                .any(|n| can_move(node, grid.get(n).unwrap()))
            {
                s.push('.');
            } else {
                s.push('#');
            }
        }
        s.push('\n');
    }
    s
}

fn calc_steps(grid: &Grid) -> usize {
    // treat this like a "fifteen puzzle"
    // get the empty tile position
    let (empty_pos, empty_avail) = grid
        .iter()
        .filter(|(_, node)| node.used == 0)
        .map(|(pos, node)| (pos, node.avail))
        .next()
        .unwrap();

    // Get the "blockers": they are fixed and cannot be moved by the empty tile.
    // For the given input, they are in a single left-to-right row segment.
    // Get the x coord (column) of the leftmost blocker, we must go around it.
    let first_free_x = grid
        .iter()
        .filter(|(_, node)| node.used > empty_avail)
        .map(|(pos, _)| pos.0)
        .min()
        .unwrap()
        - 1;

    // Go left to the rightmost column which is not blocked (x < lowest blocker)
    let steps_left = empty_pos.0 - first_free_x;

    // Go to the first row (y=0)
    let steps_up = empty_pos.1;

    // Go to the right where the empty tile is (x=x_max, y=0)
    let x_max = grid.keys().map(|pos| pos.0).max().unwrap();
    let steps_right = x_max - first_free_x;

    // Move the goal tile to the left by shifting the empty tile around it
    let shift_to_left = 5 * (x_max - 1);
    steps_left + steps_up + steps_right + shift_to_left
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        format!("{}", viable_nodes(&grid))
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);

        // the grid layout shows the way to a solution:
        // print_grid(&grid);
        format!("{}", calc_steps(&grid))
    }
}
