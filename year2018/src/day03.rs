use common::day::Day;
use regex::Regex;
use std::collections::HashSet;
use util::grid2d::Coords;

pub struct Day03 {}

struct Rectangle {
    top_left: Coords,
    size: Coords,
}

impl Rectangle {
    fn inside_coords(&self) -> impl Iterator<Item = Coords> + '_ {
        (self.top_left.x..(self.top_left.x + self.size.x))
            .map(move |x| {
                (self.top_left.y..(self.top_left.y + self.size.y)).map(move |y| Coords { x, y })
            })
            .flatten()
    }
}

fn parse_input(input: &str) -> Vec<Rectangle> {
    let re = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Rectangle {
                top_left: Coords {
                    x: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                },
                size: Coords {
                    x: caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    y: caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                },
            }
        })
        .collect()
}

fn overlapping(rectangles: &[Rectangle]) -> HashSet<Coords> {
    let mut claimed = HashSet::new();
    let mut overlapping = HashSet::new();
    for rectangle in rectangles {
        for coord in rectangle.inside_coords() {
            if !claimed.insert(coord) {
                overlapping.insert(coord);
            }
        }
    }
    overlapping
}

fn non_overlap(rectangles: &[Rectangle], overlapping: &HashSet<Coords>) -> Option<usize> {
    rectangles.iter().position(|rectangle| {
        let claim: HashSet<_> = rectangle.inside_coords().collect();
        claim.intersection(&overlapping).count() == 0
    })
}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let rectangles = parse_input(input);
        format!("{}", overlapping(&rectangles).len())
    }

    fn star2(&self, input: &str) -> String {
        let rectangles = parse_input(input);
        let overlap = overlapping(&rectangles);
        format!("{}", non_overlap(&rectangles, &overlap).unwrap() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;
        let d = Day03 {};
        assert_eq!(d.star1(input), "4");
        assert_eq!(d.star2(input), "3");
    }
}
