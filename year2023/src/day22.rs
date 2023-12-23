use std::collections::{HashSet, VecDeque};

use common::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let bricks = parse_input(input);
        let settled_bricks = settle(bricks);

        let mut can_be_disintegrated = 0;
        for i in 0..settled_bricks.len() {
            let mut safe = true;
            for settled_brick in settled_bricks.iter().skip(i + 1) {
                if settled_brick.supported_by.contains(&i) && settled_brick.supported_by.len() == 1
                {
                    safe = false;
                    break;
                }
            }

            if safe {
                can_be_disintegrated += 1;
            }
        }
        can_be_disintegrated.to_string()
    }

    fn star2(&self, input: &str) -> String {
        let bricks = parse_input(input);
        let settled_bricks = settle(bricks);

        let mut bricks_to_fall = 0;
        for i in 0..settled_bricks.len() {
            let mut supports: Vec<HashSet<usize>> = settled_bricks
                .iter()
                .map(|b| b.supported_by.clone())
                .collect();

            let mut queue = VecDeque::new();
            queue.push_back(i);

            while let Some(x) = queue.pop_front() {
                for (j, support) in supports.iter_mut().enumerate().skip(x + 1) {
                    if support.remove(&x) && support.is_empty() {
                        queue.push_back(j);
                        bricks_to_fall += 1;
                    }
                }
            }
        }

        bricks_to_fall.to_string()
    }
}

type Bricks = Vec<Brick>;

struct BrickWithSupports {
    brick: Brick,
    supported_by: HashSet<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Brick {
    from: Coords3D,
    to: Coords3D,
}

impl Brick {
    fn parse(line: &str) -> Brick {
        let (first, second) = line.split_once('~').unwrap();
        let first_spl: Vec<_> = first.split(',').collect();
        let second_spl: Vec<_> = second.split(',').collect();

        let from = Coords3D {
            x: first_spl[0].parse().unwrap(),
            y: first_spl[1].parse().unwrap(),
            z: first_spl[2].parse().unwrap(),
        };
        let to = Coords3D {
            x: second_spl[0].parse().unwrap(),
            y: second_spl[1].parse().unwrap(),
            z: second_spl[2].parse().unwrap(),
        };

        if from.x != to.x {
            assert!(from.x < to.x);
            assert_eq!(from.y, to.y);
            assert_eq!(from.z, to.z);
        } else if from.y != to.y {
            assert!(from.y < to.y);
            assert_eq!(from.x, to.x);
            assert_eq!(from.z, to.z);
        } else if from.z != to.z {
            assert!(from.z < to.z);
            assert_eq!(from.x, to.x);
            assert_eq!(from.y, to.y);
        } else {
            assert_eq!(from.x, to.x);
            assert_eq!(from.y, to.y);
            assert_eq!(from.z, to.z);
        };

        Brick { from, to }
    }

    fn overlap(&self, other: &Brick) -> bool {
        (self.from.x <= other.to.x && other.from.x <= self.to.x)
            && (self.from.y <= other.to.y && other.from.y <= self.to.y)
            && (self.from.z <= other.to.z && other.from.z <= self.to.z)
    }
}

fn settle(bricks: Bricks) -> Vec<BrickWithSupports> {
    let mut settled_bricks: Vec<BrickWithSupports> = vec![];

    for brick in &bricks {
        let mut moved_brick = BrickWithSupports {
            brick: brick.clone(),
            supported_by: HashSet::new(),
        };

        while moved_brick.brick.from.z > 1 {
            moved_brick.brick.from.z -= 1;
            moved_brick.brick.to.z -= 1;

            let mut blocked = false;
            for (j, settled_brick) in settled_bricks.iter_mut().enumerate() {
                if moved_brick.brick.overlap(&settled_brick.brick) {
                    blocked = true;
                    moved_brick.supported_by.insert(j);
                }
            }

            if blocked {
                moved_brick.brick.from.z += 1;
                moved_brick.brick.to.z += 1;
                break;
            }
        }
        settled_bricks.push(moved_brick);
    }
    settled_bricks
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coords3D {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_input(input: &str) -> Bricks {
    let mut bricks: Vec<_> = input.lines().map(Brick::parse).collect();
    // sort by min z
    bricks.sort_unstable_by(|a, b| a.from.z.min(a.to.z).cmp(&b.from.z.min(b.to.z)));
    bricks
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn ex1() {
        let d = Day22 {};
        assert_eq!(d.star1(INPUT), "5");
    }

    #[test]
    fn ex2() {
        let d = Day22 {};
        assert_eq!(d.star2(INPUT), "7");
    }

    #[test]
    fn test_overlap() {
        let brick1 = Brick::parse("1,0,1~1,2,1");
        let brick2 = Brick::parse("0,0,2~2,0,2");
        let brick3 = Brick::parse("0,0,1~2,0,1");
        assert_eq!(brick1.overlap(&brick2), false);
        assert_eq!(brick1.overlap(&brick3), true);
    }
}
