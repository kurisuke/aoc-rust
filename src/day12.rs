use crate::day::Day;
use crate::grid2d::Coords;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

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

#[derive(IntoPrimitive)]
#[repr(i64)]
enum Rotate {
    Rot90,
    Rot180,
    Rot270,
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut pos = Coords { x: 0, y: 0 };
        let mut dir = Dir::E;

        for cmd in cmds {
            match cmd {
                Cmd::N(v) => {
                    pos.y += v;
                }
                Cmd::S(v) => {
                    pos.y -= v;
                }
                Cmd::E(v) => {
                    pos.x += v;
                }
                Cmd::W(v) => {
                    pos.x -= v;
                }
                Cmd::L(r) => {
                    let cur_dir: i64 = dir.into();
                    let r_val: i64 = r.into();
                    dir = Dir::try_from((cur_dir - r_val - 1 + 4) % 4).unwrap();
                }
                Cmd::R(r) => {
                    let cur_dir: i64 = dir.into();
                    let r_val: i64 = r.into();
                    dir = Dir::try_from((cur_dir + r_val + 1 + 4) % 4).unwrap();
                }
                Cmd::F(v) => match dir {
                    Dir::N => {
                        pos.y += v;
                    }
                    Dir::S => {
                        pos.y -= v;
                    }
                    Dir::E => {
                        pos.x += v;
                    }
                    Dir::W => {
                        pos.x -= v;
                    }
                },
            };
        }

        format!("{}", pos.x.abs() + pos.y.abs())
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut pos = Coords { x: 0, y: 0 };
        let mut vec = Coords { x: 10, y: 1 };

        for cmd in cmds {
            match cmd {
                Cmd::N(v) => {
                    vec.y += v;
                }
                Cmd::S(v) => {
                    vec.y -= v;
                }
                Cmd::E(v) => {
                    vec.x += v;
                }
                Cmd::W(v) => {
                    vec.x -= v;
                }
                Cmd::L(rot) => match rot {
                    Rotate::Rot90 => {
                        vec = Coords {
                            x: -vec.y,
                            y: vec.x,
                        };
                    }
                    Rotate::Rot180 => {
                        vec = Coords {
                            x: -vec.x,
                            y: -vec.y,
                        };
                    }
                    Rotate::Rot270 => {
                        vec = Coords {
                            x: vec.y,
                            y: -vec.x,
                        };
                    }
                },
                Cmd::R(rot) => match rot {
                    Rotate::Rot90 => {
                        vec = Coords {
                            x: vec.y,
                            y: -vec.x,
                        };
                    }
                    Rotate::Rot180 => {
                        vec = Coords {
                            x: -vec.x,
                            y: -vec.y,
                        };
                    }
                    Rotate::Rot270 => {
                        vec = Coords {
                            x: -vec.y,
                            y: vec.x,
                        };
                    }
                },
                Cmd::F(n) => {
                    pos.x += n * vec.x;
                    pos.y += n * vec.y;
                }
            }
        }

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
