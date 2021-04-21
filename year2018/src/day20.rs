use common::day::Day;
use std::collections::HashMap;
use util::grid2d::Coords;

pub struct Day20 {}

fn walk(regex: &str) -> HashMap<Coords, usize> {
    let mut dists = HashMap::new();
    let mut stack = vec![];

    let mut cur_pos = Coords { x: 0, y: 0 };
    let mut cur_dist = 0;

    for c in regex.chars() {
        match c {
            '(' => {
                stack.push((cur_pos, cur_dist));
            }
            ')' => {
                let tmp = stack.pop().unwrap();
                cur_pos = tmp.0;
                cur_dist = tmp.1;
            }
            '|' => {
                let tmp = stack.last().unwrap();
                cur_pos = tmp.0;
                cur_dist = tmp.1;
            }
            'N' | 'S' | 'W' | 'E' => {
                match c {
                    'N' => {
                        cur_pos.y -= 1;
                    }
                    'S' => {
                        cur_pos.y += 1;
                    }
                    'W' => {
                        cur_pos.x -= 1;
                    }
                    'E' => {
                        cur_pos.x += 1;
                    }
                    _ => {}
                }
                cur_dist += 1;
                let entry = dists.entry(cur_pos).or_insert(cur_dist);
                *entry = (*entry).min(cur_dist);
            }
            _ => {}
        }
    }

    dists
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let dists = walk(input);
        let furthest = dists.values().max().unwrap();
        format!("{}", furthest)
    }

    fn star2(&self, input: &str) -> String {
        let dists = walk(input);
        let plus_1000 = dists.values().filter(|dist| **dist >= 1000).count();
        format!("{}", plus_1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day20 {};
        assert_eq!(d.star1("^WNE$"), "3");
        assert_eq!(d.star1("^ENWWW(NEEE|SSE(EE|N))$"), "10");
        assert_eq!(d.star1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), "18");
        assert_eq!(
            d.star1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
            "23"
        );
        assert_eq!(
            d.star1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
            "31"
        );
    }
}
