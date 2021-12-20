use common::day::Day;
use scan_fmt::scan_fmt;

pub struct Day23 {}

#[derive(Copy, Clone)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3D {
    fn dist(&self, pos: &Vec3D) -> i64 {
        (self.x - pos.x).abs() + (self.y - pos.y).abs() + (self.z - pos.z).abs()
    }
}

struct Bot {
    pos: Vec3D,
    r: i64,
}

impl Bot {
    fn others_in_range(&self, others: &[Bot]) -> usize {
        others.iter().filter(|o| self.in_range(&o.pos)).count()
    }

    fn in_range(&self, pos: &Vec3D) -> bool {
        self.dist(pos) <= self.r
    }

    fn dist(&self, pos: &Vec3D) -> i64 {
        self.pos.dist(pos)
    }
}

#[derive(Copy, Clone)]
struct Cube {
    pos: Vec3D,
    length: i64,
}

impl Cube {
    fn split(&self) -> Vec<Cube> {
        let l2 = self.length / 2;
        vec![
            Cube {
                pos: self.pos,
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x + l2,
                    y: self.pos.y,
                    z: self.pos.z,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x,
                    y: self.pos.y + l2,
                    z: self.pos.z,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x + l2,
                    y: self.pos.y + l2,
                    z: self.pos.z,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x,
                    y: self.pos.y,
                    z: self.pos.z + l2,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x + l2,
                    y: self.pos.y,
                    z: self.pos.z + l2,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x,
                    y: self.pos.y + l2,
                    z: self.pos.z + l2,
                },
                length: l2,
            },
            Cube {
                pos: Vec3D {
                    x: self.pos.x + l2,
                    y: self.pos.y + l2,
                    z: self.pos.z + l2,
                },
                length: l2,
            },
        ]
    }

    fn in_range(&self, bot: &Bot) -> bool {
        let dist_x = if bot.pos.x < self.pos.x {
            self.pos.x - bot.pos.x
        } else if bot.pos.x >= self.pos.x + self.length {
            bot.pos.x - (self.pos.x + self.length - 1)
        } else {
            0
        };

        let dist_y = if bot.pos.y < self.pos.y {
            self.pos.y - bot.pos.y
        } else if bot.pos.y >= self.pos.y + self.length {
            bot.pos.y - (self.pos.y + self.length - 1)
        } else {
            0
        };

        let dist_z = if bot.pos.z < self.pos.z {
            self.pos.z - bot.pos.z
        } else if bot.pos.z >= self.pos.z + self.length {
            bot.pos.z - (self.pos.z + self.length - 1)
        } else {
            0
        };

        dist_x + dist_y + dist_z <= bot.r
    }

    fn bots_in_range(&self, bots: &[Bot]) -> usize {
        bots.iter().filter(|b| self.in_range(b)).count()
    }
}

fn bounding_cube(bots: &[Bot]) -> Cube {
    let min_x = bots
        .iter()
        .min_by(|a, b| a.pos.x.cmp(&b.pos.x))
        .map(|b| b.pos.x)
        .unwrap();
    let max_x = bots
        .iter()
        .max_by(|a, b| a.pos.x.cmp(&b.pos.x))
        .map(|b| b.pos.x)
        .unwrap();
    let min_y = bots
        .iter()
        .min_by(|a, b| a.pos.y.cmp(&b.pos.y))
        .map(|b| b.pos.y)
        .unwrap();
    let max_y = bots
        .iter()
        .max_by(|a, b| a.pos.y.cmp(&b.pos.y))
        .map(|b| b.pos.y)
        .unwrap();
    let min_z = bots
        .iter()
        .min_by(|a, b| a.pos.z.cmp(&b.pos.z))
        .map(|b| b.pos.z)
        .unwrap();
    let max_z = bots
        .iter()
        .max_by(|a, b| a.pos.z.cmp(&b.pos.z))
        .map(|b| b.pos.z)
        .unwrap();

    let length_x = max_x - min_x;
    let length_y = max_y - min_y;
    let length_z = max_z - min_z;
    let length_max = length_x.max(length_y).max(length_z);

    let mut length_2 = 1;
    while length_2 < length_max {
        length_2 *= 2;
    }
    Cube {
        pos: Vec3D {
            x: min_x,
            y: min_y,
            z: min_z,
        },
        length: length_2,
    }
}

fn parse_input(input: &str) -> Vec<Bot> {
    input
        .lines()
        .map(|l| {
            let (x, y, z, r) =
                scan_fmt!(l, "pos=<{d},{d},{d}>, r={d}", i64, i64, i64, i64).unwrap();
            Bot {
                pos: Vec3D { x, y, z },
                r,
            }
        })
        .collect()
}

fn search(bots: &[Bot]) -> Vec<Vec3D> {
    let mut cubes = vec![bounding_cube(bots)];
    let mut length = cubes[0].length;
    while length > 1 {
        let mut next_cubes = vec![];
        for cube in cubes {
            let cube_splits = cube.split();
            for c in cube_splits {
                next_cubes.push((c, c.bots_in_range(bots)));
            }
        }
        let max_in_range = next_cubes.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        cubes = next_cubes
            .into_iter()
            .filter(|c| c.1 == max_in_range)
            .map(|c| c.0)
            .collect();
        length /= 2;
    }
    cubes.into_iter().map(|c| c.pos).collect()
}

fn find_shortest(poses: &[Vec3D]) -> i64 {
    poses
        .iter()
        .map(|p| p.dist(&Vec3D { x: 0, y: 0, z: 0 }))
        .min()
        .unwrap()
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let bots = parse_input(input);
        let max_bot = bots.iter().max_by(|a, b| a.r.cmp(&b.r)).unwrap();
        format!("{}", max_bot.others_in_range(&bots))
    }

    fn star2(&self, input: &str) -> String {
        let bots = parse_input(input);
        let best_poses = search(&bots);
        format!("{}", find_shortest(&best_poses))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#;

        let d = Day23 {};
        assert_eq!(d.star1(input), "7");
    }

    #[test]
    fn ex2() {
        let input = r#"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#;

        let d = Day23 {};
        assert_eq!(d.star2(input), "36");
    }
}
