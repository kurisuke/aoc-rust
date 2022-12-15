use common::day::Day;
use std::collections::HashSet;
use util::grid2d::Coords;

pub struct Day15 {}

struct Sensor {
    pos: Coords,
    beacon: Coords,
    closest_distance: u64,
}

impl Sensor {
    fn coverage_row(&self, row_y: i64) -> Option<Interval> {
        let half_width = self.closest_distance as i64 - (self.pos.y - row_y).unsigned_abs() as i64;
        if half_width >= 0 {
            Some(Interval(self.pos.x - half_width, self.pos.x + half_width))
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Interval(i64, i64);

fn merge(mut input: Vec<Interval>) -> Vec<Interval> {
    input.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by max first
    let mut output = vec![input.pop().unwrap()];
    while !input.is_empty() && output.last().unwrap().1 != std::i64::MAX {
        let next = input.pop().unwrap();
        let top = output.pop().unwrap();
        if next.0 <= top.1 + 1 {
            output.push(Interval(top.0, top.1.max(next.1)));
        } else {
            output.push(top);
            output.push(next);
        }
    }
    output
}

fn beacon_free_fields_in_row(sensors: &[Sensor], row_y: i64) -> usize {
    let beacon_poses: HashSet<_> = sensors
        .iter()
        .filter_map(|s| {
            if s.beacon.y == row_y {
                Some(s.beacon.x)
            } else {
                None
            }
        })
        .collect();

    let coverages: Vec<_> = sensors
        .iter()
        .filter_map(|s| s.coverage_row(row_y))
        .collect();

    let coverages = merge(coverages);

    coverages
        .iter()
        .map(|c| (c.1 - c.0 + 1) as usize)
        .sum::<usize>()
        - beacon_poses.len()
}

fn parse_input(input: &str) -> impl Iterator<Item = Sensor> + '_ {
    input.lines().map(|line| {
        let tokens: Vec<_> = line.split_whitespace().collect();

        let sensor_x = tokens[2].split('=').nth(1).unwrap().replace(',', "");
        let sensor_y = tokens[3].split('=').nth(1).unwrap().replace(':', "");
        let pos = Coords {
            x: sensor_x.parse().unwrap(),
            y: sensor_y.parse().unwrap(),
        };

        let beacon_x = tokens[8].split('=').nth(1).unwrap().replace(',', "");
        let beacon_y = tokens[9].split('=').nth(1).unwrap();
        let beacon = Coords {
            x: beacon_x.parse().unwrap(),
            y: beacon_y.parse().unwrap(),
        };

        Sensor {
            pos,
            beacon,
            closest_distance: pos.manhattan(&beacon),
        }
    })
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let sensors: Vec<_> = parse_input(input).collect();
        format!("{}", beacon_free_fields_in_row(&sensors, 2000000))
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            merge(vec![Interval(2, 5), Interval(3, 8)]),
            vec![Interval(2, 8)]
        );
        assert_eq!(
            merge(vec![Interval(-3, 4), Interval(5, 22), Interval(-2, 18)]),
            vec![Interval(-3, 22)]
        );
        assert_eq!(
            merge(vec![Interval(-3, 9), Interval(5, 8), Interval(11, 18)]),
            vec![Interval(-3, 9), Interval(11, 18)]
        );
        assert_eq!(
            merge(vec![Interval(3, 3), Interval(2, 2), Interval(1, 1)]),
            vec![Interval(1, 3)]
        );
    }

    #[test]
    fn test_coverage() {
        let sensor = Sensor {
            pos: Coords { x: 8, y: 7 },
            beacon: Coords { x: 2, y: 10 },
            closest_distance: 9,
        };

        assert_eq!(sensor.coverage_row(10), Some(Interval(2, 14)));
    }

    #[test]
    fn ex1() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

        let sensors: Vec<_> = parse_input(input).collect();
        assert_eq!(beacon_free_fields_in_row(&sensors, 10), 26);
    }
}
