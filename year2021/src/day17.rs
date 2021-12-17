use common::day::Day;
use scan_fmt::scan_fmt;
use std::cmp::Ordering;
use util::grid2d::Coords;

pub struct Day17 {}

struct TargetArea {
    min: Coords,
    max: Coords,
}

fn parse_input(input: &str) -> TargetArea {
    let (x_min, x_max, y_min, y_max) = scan_fmt!(
        input.trim(),
        "target area: x={d}..{d}, y={d}..{d}",
        i64,
        i64,
        i64,
        i64
    )
    .unwrap();
    TargetArea {
        min: Coords { x: x_min, y: y_min },
        max: Coords { x: x_max, y: y_max },
    }
}

fn trajectory(v_init: &Coords, target: &TargetArea) -> Option<i64> {
    let mut v = *v_init;
    let mut pos = Coords { x: 0, y: 0 };
    let mut max_y = 0;
    loop {
        max_y = pos.y.max(max_y);
        pos += v;

        // check if hit target area
        if pos.x >= target.min.x
            && pos.y >= target.min.y
            && pos.x <= target.max.x
            && pos.y <= target.max.y
        {
            return Some(max_y);
        }

        // update velocity vector
        v.x = match v.x.cmp(&0) {
            Ordering::Less => v.x + 1,
            Ordering::Equal => 0,
            Ordering::Greater => v.x - 1,
        };
        v.y -= 1;

        // abort condition
        if v.x == 0 && (pos.x < target.min.x || pos.x > target.max.x) {
            break;
        }
        if v.y < 0 && pos.y < target.min.y {
            break;
        }
    }
    None
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let target = parse_input(input);
        let mut max_y = 0;
        for x in 0..=target.max.x {
            for y in 0..=target.max.x {
                let v_init = Coords { x, y };
                if let Some(new_y) = trajectory(&v_init, &target) {
                    max_y = max_y.max(new_y);
                }
            }
        }
        format!("{}", max_y)
    }

    fn star2(&self, input: &str) -> String {
        let target = parse_input(input);
        let mut num_hits = 0;
        for x in -target.max.x..=target.max.x {
            for y in -target.max.x..=target.max.x {
                let v_init = Coords { x, y };
                if trajectory(&v_init, &target).is_some() {
                    num_hits += 1;
                }
            }
        }
        format!("{}", num_hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day17 {};
        assert_eq!(d.star1("target area: x=20..30, y=-10..-5"), "45");
        assert_eq!(d.star2("target area: x=20..30, y=-10..-5"), "112");
    }
}
