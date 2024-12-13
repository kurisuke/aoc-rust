use std::ops::Sub;

use common::day::Day;
use itertools::Itertools;

pub struct Day24 {}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let paths: Vec<_> = input.lines().map(Path::parse).collect();

        let range_min = 200000000000000.0;
        let range_max = 400000000000000.0;
        count_intersections_2d(&paths, (range_min, range_max), (range_min, range_max)).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let paths: Vec<_> = input.lines().map(Path::parse).collect();
        let rock = part2(&paths);
        rock.to_string()
    }
}

#[derive(Copy, Clone)]
struct Coords3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Sub for Coords3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Coords3D {
    fn dotp(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn crossp(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[allow(clippy::type_complexity)]
struct Matrix3x3 {
    m: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)),
}

impl Matrix3x3 {
    fn from_rows(v1: &Coords3D, v2: &Coords3D, v3: &Coords3D) -> Self {
        Self {
            m: ((v1.x, v1.y, v1.z), (v2.x, v2.y, v2.z), (v3.x, v3.y, v3.z)),
        }
    }

    fn det(&self) -> f64 {
        self.m.0 .0 * self.m.1 .1 * self.m.2 .2
            + self.m.0 .1 * self.m.1 .2 * self.m.2 .0
            + self.m.0 .2 * self.m.1 .0 * self.m.2 .1
            - self.m.0 .2 * self.m.1 .1 * self.m.2 .0
            - self.m.0 .0 * self.m.1 .2 * self.m.2 .1
            - self.m.0 .1 * self.m.1 .0 * self.m.2 .2
    }

    fn replace(&self, b: &Coords3D, n: usize) -> Self {
        let m = match n {
            0 => (
                (b.x, self.m.0 .1, self.m.0 .2),
                (b.y, self.m.1 .1, self.m.1 .2),
                (b.z, self.m.2 .1, self.m.2 .2),
            ),
            1 => (
                (self.m.0 .0, b.x, self.m.0 .2),
                (self.m.1 .0, b.y, self.m.1 .2),
                (self.m.2 .0, b.z, self.m.2 .2),
            ),
            2 => (
                (self.m.0 .0, self.m.0 .1, b.x),
                (self.m.1 .0, self.m.1 .1, b.y),
                (self.m.2 .0, self.m.2 .1, b.z),
            ),
            _ => unreachable!(),
        };
        Self { m }
    }
}

struct Path {
    p: Coords3D,
    v: Coords3D,
}

impl Path {
    fn parse(line: &str) -> Self {
        let (p_str, v_str) = line.split_once(" @ ").unwrap();
        let p_str: Vec<_> = p_str.split(", ").collect();
        let v_str: Vec<_> = v_str.split(", ").collect();
        let p = Coords3D {
            x: p_str[0].trim().parse().unwrap(),
            y: p_str[1].trim().parse().unwrap(),
            z: p_str[2].trim().parse().unwrap(),
        };
        let v = Coords3D {
            x: v_str[0].trim().parse().unwrap(),
            y: v_str[1].trim().parse().unwrap(),
            z: v_str[2].trim().parse().unwrap(),
        };

        Self { p, v }
    }

    fn intersect_2d_xy(&self, other: &Self) -> Option<Coords3D> {
        // line equation for self
        let a = self.v.y / self.v.x;
        let c = self.p.y - self.p.x * self.v.y / self.v.x;
        // line equation for other
        let b = other.v.y / other.v.x;
        let d = other.p.y - other.p.x * other.v.y / other.v.x;

        if a == b {
            None
        } else {
            Some(Coords3D {
                x: (d - c) / (a - b),
                y: a * (d - c) / (a - b) + c,
                z: 0.0,
            })
        }
    }

    fn intersect_2d_xz(&self, other: &Self) -> Option<Coords3D> {
        // line equation for self
        let a = self.v.z / self.v.x;
        let c = self.p.z - self.p.x * self.v.z / self.v.x;
        // line equation for other
        let b = other.v.z / other.v.x;
        let d = other.p.z - other.p.x * other.v.z / other.v.x;

        if a == b {
            None
        } else {
            Some(Coords3D {
                x: (d - c) / (a - b),
                z: a * (d - c) / (a - b) + c,
                y: 0.0,
            })
        }
    }

    fn intersect_in_future(&self, i: &Coords3D) -> bool {
        (self.v.x >= 0.0 && i.x >= self.p.x) || (self.v.x <= 0.0 && i.x <= self.p.x)
    }
}

fn count_intersections_2d(paths: &[Path], x_range: (f64, f64), y_range: (f64, f64)) -> usize {
    paths
        .iter()
        .combinations(2)
        .filter(|pair| {
            let a = pair[0];
            let b = pair[1];

            if let Some(i) = a.intersect_2d_xy(b) {
                a.intersect_in_future(&i)
                    && b.intersect_in_future(&i)
                    && i.x >= x_range.0
                    && i.x <= x_range.1
                    && i.y >= y_range.0
                    && i.y <= y_range.1
            } else {
                false
            }
        })
        .count()
}

fn part2(paths: &[Path]) -> f64 {
    let path1 = &paths[0];
    let path2 = &paths[1];
    let path3 = &paths[2];

    // thrown rock and a hailstone will intersect at some point in time
    // r = initial position of rock
    // s1, s2 = initial position of hailstone
    // r + t2 * w = s1 + t1 * v1
    // r + t2 * w = s2 + t2 * v2

    // (r - s1) = t1 * (v1 - w)
    // (r - s2) = t2 * (v2 - w)

    // consider the triangle between r, s1, s2
    // side A is (s1 - s2)
    // side B is t1 * (v1 - w)
    // side C is t2 * (v2 - w)

    // these 3 vectors are in the same plane --> their triple product is 0
    // ((v1 - w) x (v2 - w)) * (s1 - s2) = 0
    // ((v1 - w) x (v2 - v1)) * (s1 - s2) = 0
    // ((v2 - v1) x (s1 - s2)) * (v1 - w) = 0
    // ((s1 - s2) x (v1 - v2)) * (v1 - w) = 0
    // ((s1 - s2) x (v1 - v2)) * v1 = ((s1 - s2) x (v1 - v2)) * w

    // linear system of equations for rock's velocity vector: wx, wy, wz
    // g1 = ((s1 - s2) x (v1 - v2))
    // f1 = (g11, g12, g13) * v1
    // f1 = g11 * wx + g12 * wy + g13 * wz

    // g2 = ((s2 - s3) x (v2 - v3))
    // f2 = (g21, g22, g23) * v2
    // f2 = g21 * wx + g22 * wy + g23 * wz

    // g3 = ((s3 - s3) x (v3 - v1))
    // f3 = (g21, g22, g23) * v3
    // f3 = g31 * wx + g32 * wy + g33 * wz

    let g1 = (path1.p - path2.p).crossp(&(path1.v - path2.v));
    let f1 = g1.dotp(&path1.v);
    let g2 = (path2.p - path3.p).crossp(&(path2.v - path3.v));
    let f2 = g2.dotp(&path2.v);
    let g3 = (path3.p - path1.p).crossp(&(path3.v - path1.v));
    let f3 = g3.dotp(&path3.v);

    // solve with Cramer's rule for Ax = b
    // x = (wx, wy, wz)
    let a = Matrix3x3::from_rows(&g1, &g2, &g3);
    let b = Coords3D {
        x: f1,
        y: f2,
        z: f3,
    };
    let awx = a.replace(&b, 0);
    let awy = a.replace(&b, 1);
    let awz = a.replace(&b, 2);

    let wx = awx.det() / a.det();
    let wy = awy.det() / a.det();
    let wz = awz.det() / a.det();
    let w = Coords3D {
        x: wx,
        y: wy,
        z: wz,
    };

    // find r such that:
    // r = s1 + t1 * (v1 - w)
    // r = s2 + t2 * (v2 - w)
    // r will be at the intersection of both paths
    let path1 = Path {
        p: path1.p,
        v: path1.v - w,
    };
    let path2 = Path {
        p: path2.p,
        v: path2.v - w,
    };

    // find intersection points in XY, XZ, so we can combine the coordinates
    let r_xy = path1.intersect_2d_xy(&path2).unwrap();
    let r_xz = path1.intersect_2d_xz(&path2).unwrap();

    r_xy.x + r_xy.y + r_xz.z
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn star1() {
        let paths: Vec<_> = INPUT.lines().map(Path::parse).collect();
        assert_eq!(count_intersections_2d(&paths, (7.0, 27.0), (7.0, 27.0)), 2);
    }

    #[test]
    fn star2() {
        let d = Day24 {};
        assert_eq!(d.star2(INPUT), "47");
    }

    #[test]
    fn test_intersect_2d() {
        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("18, 19, 22 @ -1, -1, -2");
        let i = a.intersect_2d_xy(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert!(a.intersect_in_future(&i));
        assert!(i.x - 14.333 < 0.001);
        assert!(i.y - 15.333 < 0.001);

        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("20, 25, 34 @ -2, -2, -4");
        let i = a.intersect_2d_xy(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert!(a.intersect_in_future(&i));
        assert!(i.x - 11.667 < 0.001);
        assert!(i.y - 16.667 < 0.001);

        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("20, 19, 15 @ 1, -5, -3");
        let i = a.intersect_2d_xy(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert!(!a.intersect_in_future(&i));
        assert!(b.intersect_in_future(&i));

        let a = Path::parse("18, 19, 22 @ -1, -1, -2");
        let b = Path::parse("20, 25, 34 @ -2, -2, -4");
        let i = a.intersect_2d_xy(&b);
        assert!(i.is_none());
    }
}
