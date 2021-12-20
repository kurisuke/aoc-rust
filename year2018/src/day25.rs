use common::day::Day;
use std::collections::{BTreeSet, HashSet};

pub struct Day25 {}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd, Debug)]
struct Point {
    p: [i64; 4],
}

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (0..4).map(|i| (self.p[i] - other.p[i]).abs()).sum()
    }
}

type Constellation = BTreeSet<Point>;
type Constellations = HashSet<Constellation>;

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let v: Vec<_> = l.trim().split(',').map(|x| x.parse().unwrap()).collect();
            Point {
                p: [v[0], v[1], v[2], v[3]],
            }
        })
        .collect()
}

fn find_constellations(points: &[Point]) -> Constellations {
    let mut constellations = HashSet::new();
    for p in points {
        let joinable: Vec<_> = constellations
            .iter()
            .filter(|c: &&Constellation| c.iter().any(|o| p.dist(o) <= 3))
            .cloned()
            .collect();
        let mut united = BTreeSet::new();
        for j in joinable {
            united = united.union(&j).cloned().collect();
            constellations.remove(&j);
        }
        united.insert(*p);
        constellations.insert(united);
    }
    constellations
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let points = parse_input(input);
        format!("{}", find_constellations(&points).len())
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#" 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "2");
    }

    #[test]
    fn ex2() {
        let input = r#"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "4");
    }

    #[test]
    fn ex3() {
        let input = r#"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "3");
    }

    #[test]
    fn ex4() {
        let input = r#"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "8");
    }
}
