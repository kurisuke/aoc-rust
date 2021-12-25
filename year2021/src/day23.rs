use common::day::Day;
use std::cmp::Ordering;
use std::collections::{BinaryHeap,HashMap};

pub struct Day23 {}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Field {
    Room(char, Option<char>, Option<char>),
    Hallway(Option<char>),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Burrow {
    fields: [Field; 11],
    cost: usize,
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Burrow {
    fn new(start_pos: &[char]) -> Burrow {
        Burrow {
            fields: [
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Hallway(None),
                Field::Room('A', Some(start_pos[0]), Some(start_pos[1])),
                Field::Room('B', Some(start_pos[2]), Some(start_pos[3])),
                Field::Room('C', Some(start_pos[4]), Some(start_pos[5])),
                Field::Room('D', Some(start_pos[6]), Some(start_pos[7])),
            ],
            cost: 0,
        }
    }

    fn next_moves(&self, move_table: &MoveTable) -> Vec<Burrow> {
        let mut moves = vec![];
        for (i, f) in self.fields.iter().enumerate() {
            if let Field::Hallway(Some(p)) = f {
                // moves from hallways: only into target room
                if let Some(m) = self.move_to_room(i, *p, move_table) {
                    moves.push(m);
                }
            } else if let Field::Room(r, Some(p1), Some(p2)) = f {
                if r != p1 || r != p2 { // pod in wrong room
                    if let Some(m) = self.move_to_room(i, *p1, move_table) {
                        moves.push(m);
                    }
                    moves.extend(self.move_to_hallway(i, move_table));
                }
            } else if let Field::Room(r, None, Some(p)) = f {
                if r != p { // pod in wrong room
                    if let Some(m) = self.move_to_room(i, *p, move_table) {
                        moves.push(m);
                    }
                    moves.extend(self.move_to_hallway(i, move_table));
                }
            }
        }
        moves
    }

    fn move_to_room(&self, i: usize, p: char, move_table: &MoveTable) -> Option<Burrow> {
        // moves from hallways: only into target room
        let target_room = match p {
            'A' => 7,
            'B' => 8,
            'C' => 9,
            'D' => 10,
            _ => unreachable!(),
        };
        // room empty?
        let t = self.fields[target_room];
        if let Field::Room(_, None, None) = t {
            // can move, check intermediates
            if self.check_intermediates(i, target_room, move_table) {
                return Some(self.gen_move(i, target_room, move_table));
            }
        } else if let Field::Room(_, None, Some(o)) = t {
            if p == o { // other pod is of the same type?
                // can move, check intermediates
                if self.check_intermediates(i, target_room, move_table) {
                    return Some(self.gen_move(i, target_room, move_table));
                }
            }
        }
        None
    }

    fn move_to_hallway(&self, i: usize, move_table: &MoveTable) -> Vec<Burrow> {
        let mut ret = vec![];
        for h in 0..7 {
            if let Field::Hallway(None) = self.fields[h] {
                // can move, check intermediates
                if self.check_intermediates(i, h, move_table) {
                    ret.push(self.gen_move(i, h, move_table));
                }
            }
        }
        ret
    }

    fn check_intermediates(&self, start: usize, finish: usize, move_table: &MoveTable) -> bool {
        let fields = &move_table.get(&(start, finish)).unwrap().0;
        fields.iter().all(|k| matches!(self.fields[*k], Field::Hallway(None)))
    }

    fn gen_move(&self, start: usize, finish: usize, move_table: &MoveTable) -> Burrow {
        let mut new_state = self.clone();
        let mut add_cost = 0;

        let move_pod = if let Field::Room(r, Some(p1), Some(p2)) = self.fields[start] {
            new_state.fields[start] = Field::Room(r, None, Some(p2));
            p1
        } else if let Field::Room(r, None, Some(p)) = self.fields[start] {
            new_state.fields[start] = Field::Room(r, None, None);
            add_cost += 1;
            p
        } else if let Field::Hallway(Some(p)) = self.fields[start] {
            new_state.fields[start] = Field::Hallway(None);
            p
        } else {
            unreachable!()
        };

        if let Field::Room(r, None, Some(p2)) = self.fields[finish] {
            new_state.fields[finish] = Field::Room(r, Some(move_pod), Some(p2));
        } else if let Field::Room(r, None, None) = self.fields[finish] {
            new_state.fields[finish] = Field::Room(r, None, Some(move_pod));
            add_cost += 1;
        } else if let Field::Hallway(None) = self.fields[finish] {
            new_state.fields[finish] = Field::Hallway(Some(move_pod));
        } else {
            unreachable!()
        };

        add_cost += move_table.get(&(start, finish)).unwrap().1;
        let cost_modifier = match move_pod {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        };
        add_cost *= cost_modifier;
        // println!("{} : {} -> {}, cost: {}", move_pod, start, finish, add_cost);
        new_state.cost += add_cost;

        new_state
    }

    fn is_target_state(&self) -> bool {
        self.fields[7] == Field::Room('A', Some('A'), Some('A'))
            && self.fields[8] == Field::Room('B', Some('B'), Some('B'))
            && self.fields[9] == Field::Room('C', Some('C'), Some('C'))
            && self.fields[10] == Field::Room('D', Some('D'), Some('D'))
    }
}

fn search(init_state: &Burrow, move_table: &MoveTable) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    frontier.push(*init_state);

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(init_state.fields, init_state.cost);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.is_target_state() {
            return Some(current.cost);
        }

        for next in current.next_moves(move_table) {
            if !cost_so_far.contains_key(&next.fields) || next.cost < *cost_so_far.get(&next.fields).unwrap() {
                cost_so_far.insert(next.fields, next.cost);
                frontier.push(next);
            }
        }
    }
    
    None
}

type MoveTable = HashMap<(usize, usize), (Vec<usize>, usize)>;

const MOVE_TABLE_STATIC: [(usize, usize, &str, usize); 40] = [
    (0, 7, "1", 3),
    (0, 8, "1,2", 5),
    (0, 9, "1,2,3", 7),
    (0, 10, "1,2,3,4", 9),

    (1, 7, "", 2),
    (1, 8, "2", 4),
    (1, 9, "2,3", 6),
    (1, 10, "2,3,4", 8),

    (2, 7, "", 2),
    (2, 8, "", 2),
    (2, 9, "3", 4),
    (2, 10, "3,4", 6),

    (3, 7, "2", 4),
    (3, 8, "", 2),
    (3, 9, "", 2),
    (3, 10, "4", 4),

    (4, 7, "2,3", 6),
    (4, 8, "3", 4),
    (4, 9, "", 2),
    (4, 10, "", 2),

    (5, 7, "2,3,4", 8),
    (5, 8, "3,4", 6),
    (5, 9, "4", 4),
    (5, 10, "", 2),

    (6, 7, "2,3,4,5", 9),
    (6, 8, "3,4,5", 7),
    (6, 9, "4,5", 5),
    (6, 10, "5", 3),

    (7, 8, "2", 4),
    (7, 9, "2,3", 6),
    (7, 10, "2,3,4", 8),

    (8, 7, "2", 4),
    (8, 9, "3", 4),
    (7, 10, "3,4", 6),

    (9, 7, "2,3", 6),
    (9, 8, "3", 4),
    (9, 10, "4", 4),

    (10, 7, "2,3,4", 8),
    (10, 8, "3,4", 6),
    (10, 9, "4", 4),
];

fn gen_move_table() -> MoveTable {
    let mut ret = HashMap::new();
    for l in MOVE_TABLE_STATIC.iter() {
        ret.insert((l.0, l.1), (l.2.split(',').filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect() , l.3));
        ret.insert((l.1, l.0), (l.2.split(',').filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect() , l.3));
    }
    ret
}

fn parse_input(input: &str) -> Vec<char> {
    let l1 = input.lines().nth(2).unwrap();
    let l1c: Vec<_> = l1.chars().collect();

    let l2 = input.lines().nth(3).unwrap();
    let l2c: Vec<_> = l2.chars().collect();

    vec![l1c[3], l2c[3], l1c[5], l2c[5], l1c[7], l2c[7], l1c[9], l2c[9]]
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let init_pods = parse_input(input);
        let init_state = Burrow::new(&init_pods);

        let move_table = gen_move_table();
        let cost = search(&init_state, &move_table).unwrap();
        
        format!("{}", cost)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day23 {};
        let input = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;
        assert_eq!(d.star1(input), "12521");
    }
}
