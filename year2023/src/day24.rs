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

    fn star2(&self, _input: &str) -> String {
        // sagemath:
        //
        // var('px vx py vy pz vz t0 t1 t2')
        // eq1 = px + t0 * vx == 386183914429810 + t0 * 6
        // eq2 = py + t0 * vy == 203234597957945 + t0 * 106
        // eq3 = pz + t0 * vz == 537104238090859 + t0 * (-164)
        //
        // eq4 = px + t1 * vx == 191853805235172 + t1 * 205
        // eq5 = py + t1 * vy == 96933297552275 + t1 * 517
        // eq6 = pz + t1 * vz == 142797538377781 + t1 * 229
        //
        // eq7 = px + t2 * vx == 447902097938436 + t2 * (-136)
        // eq8 = py + t2 * vy == 262258252263185 + t2 * 38
        // eq9 = pz + t2 * vz == 255543483328939 + t2 * 89
        //
        // solve([eq1, eq2, eq3, eq4, eq5, eq6, eq7, eq8, eq9], px, vx, py, vy, pz, vz, t0, t1, t2)

        String::from("not implemented")
    }
}

struct Coords3D {
    x: f64,
    y: f64,
    _z: f64,
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
            _z: p_str[2].trim().parse().unwrap(),
        };
        let v = Coords3D {
            x: v_str[0].trim().parse().unwrap(),
            y: v_str[1].trim().parse().unwrap(),
            _z: v_str[2].trim().parse().unwrap(),
        };

        Self { p, v }
    }

    fn intersect_2d(&self, other: &Self) -> Option<Coords3D> {
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
                _z: 0.0,
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

            if let Some(i) = a.intersect_2d(b) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

        let paths: Vec<_> = input.lines().map(Path::parse).collect();
        assert_eq!(count_intersections_2d(&paths, (7.0, 27.0), (7.0, 27.0)), 2);
    }

    #[test]
    fn test_intersect_2d() {
        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("18, 19, 22 @ -1, -1, -2");
        let i = a.intersect_2d(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(a.intersect_in_future(&i), true);
        assert!(i.x - 14.333 < 0.001);
        assert!(i.y - 15.333 < 0.001);

        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("20, 25, 34 @ -2, -2, -4");
        let i = a.intersect_2d(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(a.intersect_in_future(&i), true);
        assert!(i.x - 11.667 < 0.001);
        assert!(i.y - 16.667 < 0.001);

        let a = Path::parse("19, 13, 30 @ -2, 1, -2");
        let b = Path::parse("20, 19, 15 @ 1, -5, -3");
        let i = a.intersect_2d(&b);
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(a.intersect_in_future(&i), false);
        assert_eq!(b.intersect_in_future(&i), true);

        let a = Path::parse("18, 19, 22 @ -1, -1, -2");
        let b = Path::parse("20, 25, 34 @ -2, -2, -4");
        let i = a.intersect_2d(&b);
        assert!(i.is_none());
    }
}
