use common::day::Day;
use util::grid2d::Coords;

pub struct Day06 {}

fn parse_input(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(", ").collect();
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
                .into()
        })
        .collect()
}

fn get_closest(pos: &Coords, points: &[Coords]) -> Option<usize> {
    let min_dist = points.iter().map(|p| pos.manhattan(p)).min().unwrap();
    let min_positions: Vec<_> = points
        .iter()
        .enumerate()
        .filter(|(_, p)| pos.manhattan(p) == min_dist)
        .map(|(idx, _)| idx)
        .collect();
    if min_positions.len() == 1 {
        Some(min_positions[0])
    } else {
        None
    }
}

fn partition(points: &[Coords]) -> Vec<Vec<Coords>> {
    // get boundaries
    let x_min = points.iter().map(|a| a.x).min().unwrap();
    let x_max = points.iter().map(|a| a.x).max().unwrap();
    let y_min = points.iter().map(|a| a.y).min().unwrap();
    let y_max = points.iter().map(|a| a.y).max().unwrap();

    let mut areas = vec![vec![]; points.len()];

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let pos = Coords { x, y };
            if let Some(closest) = get_closest(&pos, points) {
                areas[closest].push(pos);
            }
        }
    }

    // filter border coordinates
    for area in areas.iter_mut() {
        if area
            .iter()
            .any(|c| c.x == x_min || c.x == x_max || c.y == y_min || c.y == y_max)
        {
            area.clear();
        }
    }

    areas
}

fn best_area(points: &[Coords], max_sum: u64) -> Vec<Coords> {
    // get boundaries
    let x_min = points.iter().map(|a| a.x).min().unwrap();
    let x_max = points.iter().map(|a| a.x).max().unwrap();
    let y_min = points.iter().map(|a| a.y).min().unwrap();
    let y_max = points.iter().map(|a| a.y).max().unwrap();

    let mut area = vec![];
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let pos = Coords { x, y };
            let sum_dist = points.iter().map(|p| pos.manhattan(p)).sum::<u64>();
            if sum_dist < max_sum {
                area.push(pos);
            }
        }
    }
    area
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let points = parse_input(input);
        let areas = partition(&points);
        let max_area = areas.iter().map(|a| a.len()).max().unwrap();
        format!("{}", max_area)
    }

    fn star2(&self, input: &str) -> String {
        let points = parse_input(input);
        format!("{}", best_area(&points, 10000).len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day06 {};
        let input = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;
        assert_eq!(d.star1(input), "17");

        let points = parse_input(input);
        assert_eq!(best_area(&points, 32).len(), 16);
    }
}
