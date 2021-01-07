use common::day::Day;
use util::grid2d::{Direction, Grid2D};

pub struct Day19 {}

fn follow(grid: &Grid2D<char>) -> (String, usize) {
    let mut found_letters = String::new();
    let mut pos = grid.enumerate().find(|(_, c)| **c == '|').unwrap().0;
    let mut dir = Direction::S;
    let mut steps = 0;
    while let Some(c) = grid.at(&pos) {
        match c {
            '|' | '-' => {}
            '+' => {
                if dir == Direction::S || dir == Direction::N {
                    let w = grid.at(&pos.mov(Direction::W));
                    let e = grid.at(&pos.mov(Direction::E));
                    if w.unwrap_or(&' ') != &' ' {
                        dir = Direction::W;
                    } else if e.unwrap_or(&' ') != &' ' {
                        dir = Direction::E;
                    } else {
                        panic!("No way from: {:?}", pos);
                    }
                } else {
                    let n = grid.at(&pos.mov(Direction::N));
                    let s = grid.at(&pos.mov(Direction::S));
                    if n.unwrap_or(&' ') != &' ' {
                        dir = Direction::N;
                    } else if s.unwrap_or(&' ') != &' ' {
                        dir = Direction::S;
                    } else {
                        panic!("No way from: {:?}", pos);
                    }
                }
            }
            'A'..='Z' => {
                found_letters.push(*c);
            }
            _ => {
                break; // End of line
            }
        }
        pos = pos.mov(dir);
        steps += 1;
    }
    (found_letters, steps)
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        follow(&grid).0
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        format!("{}", follow(&grid).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day19 {};
        let input = r#"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ "#;
        assert_eq!(d.star1(input), "ABCDEF");
        assert_eq!(d.star2(input), "38");
    }
}
