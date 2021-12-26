use common::day::Day;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day18 {}

#[derive(Debug)]
struct KeyPath {
    dist: usize,
    doors: HashSet<char>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct StateKey {
    pos: char,
    keys: BTreeSet<char>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct SearchState {
    pos: char,
    keys: BTreeSet<char>,
    dist: usize,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct StateKeyPt2 {
    pos: [char; 4],
    keys: BTreeSet<char>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct SearchStatePt2 {
    pos: [char; 4],
    keys: BTreeSet<char>,
    dist: usize,
}

impl Ord for SearchStatePt2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for SearchStatePt2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type KeyPaths = HashMap<(char, char), KeyPath>;

fn find_all_keys(grid: &Grid2D<char>) -> HashMap<char, Coords> {
    grid.coords_iter()
        .map(|c| {
            let v = grid.at(&c).unwrap();
            if v == &'@' || (v.is_alphabetic() && v.is_lowercase()) {
                Some((*v, c))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn paths_between_keys(
    grid: &Grid2D<char>,
    start_pos: &Coords,
    keys: &HashMap<char, Coords>,
) -> KeyPaths {
    let max_key = keys.keys().max().unwrap();
    let mut key_paths = HashMap::new();

    // @ (start position)
    let keys_to_find: HashSet<char> = ('a'..=*max_key).into_iter().collect();
    key_paths.extend(bfs_keys(grid, &'@', start_pos, &keys_to_find).drain());

    // letters
    for i in 'a'..=*max_key {
        let keys_to_find: HashSet<char> =
            (((i as u8 + 1) as char)..=*max_key).into_iter().collect();
        if !keys_to_find.is_empty() {
            key_paths.extend(bfs_keys(grid, &i, keys.get(&i).unwrap(), &keys_to_find).drain());
        }
    }
    key_paths
}

fn bfs_keys(
    grid: &Grid2D<char>,
    start_key: &char,
    start_pos: &Coords,
    keys_to_find: &HashSet<char>,
) -> KeyPaths {
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();

    frontier.push_back((
        *start_pos,
        KeyPath {
            dist: 0,
            doors: HashSet::new(),
        },
    ));
    visited.insert(*start_pos);

    let mut keys_found = HashMap::new();

    while let Some((cur_pos, cur_state)) = frontier.pop_front() {
        let mut frontier_new = vec![];
        for npos in grid.neighbors_cardinal_coords(&cur_pos) {
            if visited.insert(npos) {
                if let Some(v) = grid.at(&npos) {
                    match v {
                        '.' | '@' => {
                            frontier_new.push((
                                npos,
                                KeyPath {
                                    dist: cur_state.dist + 1,
                                    doors: cur_state.doors.clone(),
                                },
                            ));
                        }
                        'a'..='z' => {
                            // key
                            if keys_to_find.contains(v)
                                && !keys_found.contains_key(&(*start_key, *v))
                            {
                                keys_found.insert(
                                    (*start_key, *v),
                                    KeyPath {
                                        dist: cur_state.dist + 1,
                                        doors: cur_state.doors.clone(),
                                    },
                                );
                            }
                            frontier_new.push((
                                npos,
                                KeyPath {
                                    dist: cur_state.dist + 1,
                                    doors: cur_state.doors.clone(),
                                },
                            ));
                        }
                        'A'..='Z' => {
                            // door
                            let mut new_doors = cur_state.doors.clone();
                            new_doors.insert(*v);
                            frontier_new.push((
                                npos,
                                KeyPath {
                                    dist: cur_state.dist + 1,
                                    doors: new_doors,
                                },
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
        frontier.extend(frontier_new.drain(..));
    }
    keys_found
}

fn search_all_keys(paths: &KeyPaths, keys_to_find: &BTreeSet<char>) -> Option<usize> {
    let init_state = SearchState {
        pos: '@',
        keys: BTreeSet::new(),
        dist: 0,
    };

    let mut dist_so_far = HashMap::new();
    dist_so_far.insert(
        StateKey {
            pos: init_state.pos,
            keys: init_state.keys.clone(),
        },
        init_state.dist,
    );

    let mut frontier = BinaryHeap::new();
    frontier.push(init_state);

    while let Some(cur) = frontier.pop() {
        if &cur.keys == keys_to_find {
            return Some(cur.dist);
        }

        for next_key in keys_to_find.difference(&cur.keys) {
            let path_key = (cur.pos.min(*next_key), cur.pos.max(*next_key));

            if let Some(path) = paths.get(&path_key) {
                // check if we have all keys for the intermediate doors
                if path
                    .doors
                    .iter()
                    .all(|d| cur.keys.contains(&d.to_ascii_lowercase()))
                {
                    let mut next_keys = cur.keys.clone();
                    next_keys.insert(*next_key);
                    let next_sk = StateKey {
                        pos: *next_key,
                        keys: next_keys,
                    };
                    let next_dist = cur.dist + path.dist;
                    if !dist_so_far.contains_key(&next_sk)
                        || next_dist < *dist_so_far.get(&next_sk).unwrap()
                    {
                        let next_state = SearchState {
                            pos: next_sk.pos,
                            keys: next_sk.keys.clone(),
                            dist: next_dist,
                        };
                        dist_so_far.insert(next_sk, next_dist);
                        frontier.push(next_state);
                    }
                }
            }
        }
    }
    None
}

fn search_all_keys_pt2(paths: &[KeyPaths], keys_to_find: &BTreeSet<char>) -> Option<usize> {
    let init_state = SearchStatePt2 {
        pos: ['@'; 4],
        keys: BTreeSet::new(),
        dist: 0,
    };

    let mut dist_so_far = HashMap::new();
    dist_so_far.insert(
        StateKeyPt2 {
            pos: init_state.pos,
            keys: init_state.keys.clone(),
        },
        init_state.dist,
    );

    let mut frontier = BinaryHeap::new();
    frontier.push(init_state);

    while let Some(cur) = frontier.pop() {
        if &cur.keys == keys_to_find {
            return Some(cur.dist);
        }

        for next_key in keys_to_find.difference(&cur.keys) {
            for k in 0..4 {
                let path_key = (cur.pos[k].min(*next_key), cur.pos[k].max(*next_key));

                if let Some(path) = paths[k].get(&path_key) {
                    // check if we have all keys for the intermediate doors
                    if path
                        .doors
                        .iter()
                        .all(|d| cur.keys.contains(&d.to_ascii_lowercase()))
                    {
                        let mut next_keys = cur.keys.clone();
                        next_keys.insert(*next_key);
                        let mut next_pos = cur.pos;
                        next_pos[k] = *next_key;
                        let next_sk = StateKeyPt2 {
                            pos: next_pos,
                            keys: next_keys,
                        };
                        let next_dist = cur.dist + path.dist;
                        if !dist_so_far.contains_key(&next_sk)
                            || next_dist < *dist_so_far.get(&next_sk).unwrap()
                        {
                            let next_state = SearchStatePt2 {
                                pos: next_sk.pos,
                                keys: next_sk.keys.clone(),
                                dist: next_dist,
                            };
                            dist_so_far.insert(next_sk, next_dist);
                            frontier.push(next_state);
                        }
                    }
                }
            }
        }
    }
    None
}

fn modify_grid_pt2(grid: &mut Grid2D<char>) -> Vec<Coords> {
    let start_pos = grid.find('@').unwrap();
    grid.set(&start_pos, '#');
    grid.set(&start_pos.mov(Direction::N), '#');
    grid.set(&start_pos.mov(Direction::S), '#');
    grid.set(&start_pos.mov(Direction::E), '#');
    grid.set(&start_pos.mov(Direction::W), '#');
    grid.set(&start_pos.mov(Direction::NW), '@');
    grid.set(&start_pos.mov(Direction::NE), '@');
    grid.set(&start_pos.mov(Direction::SW), '@');
    grid.set(&start_pos.mov(Direction::SE), '@');
    vec![
        start_pos.mov(Direction::NW),
        start_pos.mov(Direction::NE),
        start_pos.mov(Direction::SW),
        start_pos.mov(Direction::SE),
    ]
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let all_keys = find_all_keys(&grid);
        let paths = paths_between_keys(&grid, all_keys.get(&'@').unwrap(), &all_keys);
        let keys_to_find: BTreeSet<_> = all_keys.keys().filter(|k| k != &&'@').cloned().collect();
        let min_dist = search_all_keys(&paths, &keys_to_find).unwrap();
        format!("{}", min_dist)
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let all_keys = find_all_keys(&grid);

        let start_poses = modify_grid_pt2(&mut grid);

        let mut paths = vec![];
        for start_pos in start_poses.iter() {
            paths.push(paths_between_keys(&grid, start_pos, &all_keys));
        }
        let keys_to_find: BTreeSet<_> = all_keys.keys().filter(|k| k != &&'@').cloned().collect();
        let min_dist = search_all_keys_pt2(&paths, &keys_to_find).unwrap();
        format!("{}", min_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#;

        let d = Day18 {};
        assert_eq!(d.star1(input), "132");
    }

    #[test]
    fn ex2() {
        let input = r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"#;

        let d = Day18 {};
        assert_eq!(d.star1(input), "136");
    }

    #[test]
    fn ex3() {
        let input = r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"#;

        let d = Day18 {};
        assert_eq!(d.star1(input), "81");
    }
}
