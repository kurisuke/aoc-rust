use common::day::Day;

use std::collections::{HashMap, HashSet};

use util::grid2d::Coords;

pub struct Day03 {}

struct Cmd {
    dir: Coords,
    dist: usize,
}

impl Cmd {
    fn from(input: &str) -> Cmd {
        let dir = match input.chars().next().unwrap() {
            'U' => Coords { x: 0, y: 1 },
            'D' => Coords { x: 0, y: -1 },
            'L' => Coords { x: -1, y: 0 },
            'R' => Coords { x: 1, y: 0 },
            _ => Coords { x: 0, y: 0 },
        };
        let dist = input[1..].parse::<usize>().unwrap();
        Cmd { dir, dist }
    }
}

type WirePath = HashMap<Coords, usize>;
type Intersections = HashMap<Coords, (usize, usize)>;

fn wire_path(cmds: &[Cmd]) -> WirePath {
    let mut path = HashMap::new();
    let mut pos = Coords { x: 0, y: 0 };
    let mut dist = 0;
    for cmd in cmds {
        for _ in 0..cmd.dist {
            dist += 1;
            pos += cmd.dir;
            path.insert(pos, dist);
        }
    }
    path
}

fn find_intersections(path1: &WirePath, path2: &WirePath) -> Intersections {
    let pos1: HashSet<_> = path1.keys().collect();
    let pos2: HashSet<_> = path2.keys().collect();
    pos1.intersection(&pos2)
        .map(|c| (**c, (*path1.get(c).unwrap(), *path2.get(c).unwrap())))
        .collect()
}

fn parse_input(input: &str) -> (Vec<Cmd>, Vec<Cmd>) {
    let line1 = input.lines().next().unwrap();
    let cmds1: Vec<_> = line1.split(',').map(Cmd::from).collect();

    let line2 = input.lines().nth(1).unwrap();
    let cmds2: Vec<_> = line2.split(',').map(Cmd::from).collect();

    (cmds1, cmds2)
}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let (cmds1, cmds2) = parse_input(input);
        let path1 = wire_path(&cmds1);
        let path2 = wire_path(&cmds2);
        let intersections = find_intersections(&path1, &path2);

        const ORIGIN: Coords = Coords { x: 0, y: 0 };
        let min_distance = intersections
            .keys()
            .map(|c| c.manhattan(&ORIGIN))
            .min()
            .unwrap();
        format!("{}", min_distance)
    }

    fn star2(&self, input: &str) -> String {
        let (cmds1, cmds2) = parse_input(input);
        let path1 = wire_path(&cmds1);
        let path2 = wire_path(&cmds2);
        let intersections = find_intersections(&path1, &path2);

        let min_steps = intersections.values().map(|d| d.0 + d.1).min().unwrap();
        format!("{}", min_steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;

        let d = Day03 {};
        assert_eq!(d.star1(input), "6");
        assert_eq!(d.star2(input), "30");
    }

    #[test]
    fn ex2() {
        let input = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

        let d = Day03 {};
        assert_eq!(d.star1(input), "159");
        assert_eq!(d.star2(input), "610");
    }

    #[test]
    fn ex3() {
        let input = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

        let d = Day03 {};
        assert_eq!(d.star1(input), "135");
        assert_eq!(d.star2(input), "410");
    }
}
