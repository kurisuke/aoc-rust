use common::day::Day;
use std::collections::{HashMap, HashSet, VecDeque};
use util::grid2d::{Coords, Grid2D};

pub struct Day20 {}

type Portals = HashMap<Coords, Coords>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct MazePos {
    pos: Coords,
    level: usize,
}

struct Maze {
    grid: Grid2D<char>,
    inner_to_outer: Portals,
    outer_to_inner: Portals,
    start: MazePos,
    end: MazePos,
}

fn find_inner_square(grid: &Grid2D<char>) -> (i64, i64, i64, i64) {
    // row
    let mut y_min = None;
    let mut y_max = None;
    for y in 2..=(grid.height() - 2) {
        let num_spaces = (2..(grid.width() - 2))
            .filter(|x| grid.at(&Coords { x: *x, y }).unwrap() == &' ')
            .count();
        if num_spaces > 4 && y_min.is_none() {
            y_min = Some(y);
        } else if num_spaces <= 4 && y_min.is_some() {
            y_max = Some(y);
            break;
        }
    }

    // column
    let mut x_min = None;
    let mut x_max = None;
    for x in 2..=(grid.width() - 2) {
        let num_spaces = (2..(grid.height() - 2))
            .filter(|y| grid.at(&Coords { x, y: *y }).unwrap() == &' ')
            .count();
        if num_spaces > 4 && x_min.is_none() {
            x_min = Some(x);
        } else if num_spaces <= 4 && x_min.is_some() {
            x_max = Some(x);
            break;
        }
    }

    // intervals are half-open: [x_min, x_max)
    (
        x_min.unwrap(),
        x_max.unwrap(),
        y_min.unwrap(),
        y_max.unwrap(),
    )
}

fn find_outer_labels(grid: &Grid2D<char>) -> HashMap<String, Coords> {
    let mut labels = HashMap::new();

    // top
    for x in 2..=(grid.width() - 2) {
        let c1 = grid.at(&Coords { x, y: 0 }).unwrap();
        let c2 = grid.at(&Coords { x, y: 1 }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x, y: 2 });
        }
    }

    // bottom
    for x in 2..=(grid.width() - 2) {
        let c1 = grid
            .at(&Coords {
                x,
                y: grid.height() - 2,
            })
            .unwrap();
        let c2 = grid
            .at(&Coords {
                x,
                y: grid.height() - 1,
            })
            .unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(
                label,
                Coords {
                    x,
                    y: grid.height() - 3,
                },
            );
        }
    }

    // left
    for y in 2..=(grid.height() - 2) {
        let c1 = grid.at(&Coords { x: 0, y }).unwrap();
        let c2 = grid.at(&Coords { x: 1, y }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x: 2, y });
        }
    }

    // right
    for y in 2..=(grid.height() - 2) {
        let c1 = grid
            .at(&Coords {
                x: grid.width() - 2,
                y,
            })
            .unwrap();
        let c2 = grid
            .at(&Coords {
                x: grid.width() - 1,
                y,
            })
            .unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(
                label,
                Coords {
                    x: grid.width() - 3,
                    y,
                },
            );
        }
    }

    labels
}

fn find_inner_labels(
    grid: &Grid2D<char>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
) -> HashMap<String, Coords> {
    let mut labels = HashMap::new();

    // top
    for x in x_min..x_max {
        let c1 = grid.at(&Coords { x, y: y_min }).unwrap();
        let c2 = grid.at(&Coords { x, y: y_min + 1 }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x, y: y_min - 1 });
        }
    }

    // bottom
    for x in x_min..x_max {
        let c1 = grid.at(&Coords { x, y: y_max - 2 }).unwrap();
        let c2 = grid.at(&Coords { x, y: y_max - 1 }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x, y: y_max });
        }
    }

    // left
    for y in y_min..y_max {
        let c1 = grid.at(&Coords { x: x_min, y }).unwrap();
        let c2 = grid.at(&Coords { x: x_min + 1, y }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x: x_min - 1, y });
        }
    }

    // right
    for y in y_min..y_max {
        let c1 = grid.at(&Coords { x: x_max - 2, y }).unwrap();
        let c2 = grid.at(&Coords { x: x_max - 1, y }).unwrap();
        if c1.is_alphabetic() && c2.is_alphabetic() {
            let label = format!("{}{}", c1, c2);
            labels.insert(label, Coords { x: x_max, y });
        }
    }

    labels
}

fn build_maze(input: &str) -> Maze {
    let grid = Grid2D::new(input).unwrap();

    let (x_min, x_max, y_min, y_max) = find_inner_square(&grid);
    let outer_labels = find_outer_labels(&grid);
    let inner_labels = find_inner_labels(&grid, x_min, x_max, y_min, y_max);

    assert_eq!(outer_labels.len(), inner_labels.len() + 2);

    let mut inner_to_outer = HashMap::new();
    let mut outer_to_inner = HashMap::new();
    for (k, inner_pos) in inner_labels.iter() {
        let outer_pos = outer_labels.get(k).unwrap();
        inner_to_outer.insert(*inner_pos, *outer_pos);
        outer_to_inner.insert(*outer_pos, *inner_pos);
    }
    let start = MazePos {
        pos: *outer_labels.get("AA").unwrap(),
        level: 0,
    };
    let end = MazePos {
        pos: *outer_labels.get("ZZ").unwrap(),
        level: 0,
    };
    Maze {
        grid,
        inner_to_outer,
        outer_to_inner,
        start,
        end,
    }
}

fn search(maze: &Maze, recursive: bool) -> Option<usize> {
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();

    frontier.push_back((maze.start, 0));
    visited.insert(maze.start);

    while let Some((cur_pos, cur_dist)) = frontier.pop_front() {
        // end point reached
        if cur_pos == maze.end {
            return Some(cur_dist);
        }

        // regular neighbors
        for next_pos in maze
            .grid
            .neighbors_cardinal_coords(&cur_pos.pos)
            .into_iter()
            .filter(|c| maze.grid.at(c).unwrap() == &'.')
        {
            let next_pos = MazePos {
                pos: next_pos,
                level: cur_pos.level,
            };
            if visited.insert(next_pos) {
                frontier.push_back((next_pos, cur_dist + 1));
            }
        }

        // portal: inner to outer
        if maze.inner_to_outer.contains_key(&cur_pos.pos) {
            let next_pos = MazePos {
                pos: *maze.inner_to_outer.get(&cur_pos.pos).unwrap(),
                level: if recursive { cur_pos.level + 1 } else { 0 },
            };
            if visited.insert(next_pos) {
                frontier.push_back((next_pos, cur_dist + 1));
            }
        }

        // portal: outer to inner
        if maze.outer_to_inner.contains_key(&cur_pos.pos) && (!recursive || cur_pos.level > 0) {
            let next_pos = MazePos {
                pos: *maze.outer_to_inner.get(&cur_pos.pos).unwrap(),
                level: if recursive { cur_pos.level - 1 } else { 0 },
            };
            if visited.insert(next_pos) {
                frontier.push_back((next_pos, cur_dist + 1));
            }
        }
    }
    None
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let maze = build_maze(input);
        format!("{}", search(&maze, false).unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let maze = build_maze(input);
        format!("{}", search(&maze, true).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "#;

        let d = Day20 {};
        assert_eq!(d.star1(input), "58");
    }

    #[test]
    fn ex2() {
        let input = r#"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "#;

        let d = Day20 {};
        assert_eq!(d.star2(input), "396");
    }
}
