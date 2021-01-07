use common::day::Day;
use util::grid2d::Coords;

pub struct Day12 {}

enum Cmd {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(Rotate),
    R(Rotate),
    F(i64),
}

enum Rotate {
    Rot90,
    Rot180,
    Rot270,
}

fn rot_l(c: &Coords, r: Rotate) -> Coords {
    match r {
        Rotate::Rot90 => Coords { x: -c.y, y: c.x },
        Rotate::Rot180 => Coords { x: -c.x, y: -c.y },
        Rotate::Rot270 => Coords { x: c.y, y: -c.x },
    }
}

fn rot_r(c: &Coords, r: Rotate) -> Coords {
    match r {
        Rotate::Rot90 => Coords { x: c.y, y: -c.x },
        Rotate::Rot180 => Coords { x: -c.x, y: -c.y },
        Rotate::Rot270 => Coords { x: -c.y, y: c.x },
    }
}

fn move_ship(cmds: Vec<Cmd>, init_vec: Coords, direct: bool) -> Coords {
    let mut pos = Coords { x: 0, y: 0 };
    let mut vec = init_vec;

    for cmd in cmds {
        let mv = if direct { &mut pos } else { &mut vec };

        match cmd {
            Cmd::N(v) => mv.y += v,
            Cmd::S(v) => mv.y -= v,
            Cmd::E(v) => mv.x += v,
            Cmd::W(v) => mv.x -= v,
            Cmd::L(rot) => vec = rot_l(&vec, rot),
            Cmd::R(rot) => vec = rot_r(&vec, rot),
            Cmd::F(n) => {
                pos.x += n * vec.x;
                pos.y += n * vec.y;
            }
        };
    }

    pos
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let pos = move_ship(cmds, Coords { x: 1, y: 0 }, true);

        format!("{}", pos.x.abs() + pos.y.abs())
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let pos = move_ship(cmds, Coords { x: 10, y: 1 }, false);

        format!("{}", pos.x.abs() + pos.y.abs())
    }
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| {
            let cmd = l.chars().next().unwrap();
            let param = &l[1..].parse::<i64>().unwrap();
            match cmd {
                'N' => Cmd::N(*param),
                'S' => Cmd::S(*param),
                'E' => Cmd::E(*param),
                'W' => Cmd::W(*param),
                'L' => Cmd::L(parse_rotate_param(*param)),
                'R' => Cmd::R(parse_rotate_param(*param)),
                'F' => Cmd::F(*param),
                _ => {
                    panic!("Wrong cmd: {}", cmd);
                }
            }
        })
        .collect()
}

fn parse_rotate_param(param: i64) -> Rotate {
    match param {
        90 => Rotate::Rot90,
        180 => Rotate::Rot180,
        270 => Rotate::Rot270,
        _ => {
            panic!("Wrong rotate param: {}", param);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"F10
N3
F7
R90
F11"#;
        let d = Day12 {};
        assert_eq!(d.star1(input), "25");
        assert_eq!(d.star2(input), "286");
    }
}
