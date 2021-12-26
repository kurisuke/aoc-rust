use common::day::Day;
use std::collections::{HashMap, HashSet};
use util::grid2d::{Coords, Direction};
use util::intcode::{IntSize, Intcode};

pub struct Day15 {}

type Maze = HashMap<Coords, char>;
type Visited = HashSet<Coords>;
type Path = Vec<Direction>;
type FullPaths = Vec<Vec<Direction>>;

struct MazeRobot {
    intcode: Intcode,
}

impl MazeRobot {
    fn mov(&mut self, dir: &Direction) -> IntSize {
        let inp = match dir {
            Direction::N => 1,
            Direction::S => 2,
            Direction::W => 3,
            Direction::E => 4,
            _ => unreachable!(),
        };
        self.intcode.write_inp(inp);
        self.intcode.run();
        self.intcode.read_outp().unwrap()
    }

    fn mov_back(&mut self, dir: &Direction) {
        let oppo_dir = match dir {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
            _ => unreachable!(),
        };
        self.mov(&oppo_dir);
    }
}

fn try_moves(
    pos: Coords,
    robot: &mut MazeRobot,
    maze: &mut Maze,
    visited: &mut Visited,
    path: &mut Path,
    full_paths: &mut FullPaths,
) {
    if visited.contains(&pos) {
        return;
    }
    visited.insert(pos);

    for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
        let new_pos = pos.mov(dir);
        if !visited.contains(&new_pos) && maze.get(&new_pos).unwrap_or(&'.') != &'#' {
            let move_result = robot.mov(&dir);

            match move_result {
                0 => {
                    // hit a wall, move has failed
                    maze.insert(new_pos, '#');
                }
                1 => {
                    // move was successful
                    path.push(dir);
                    maze.insert(new_pos, '.');

                    try_moves(new_pos, robot, maze, visited, path, full_paths);

                    // when the moves are exhausted, move the robot back to the field before
                    path.pop();
                    robot.mov_back(&dir);
                }
                2 => {
                    // reached the end field
                    path.push(dir);
                    maze.insert(new_pos, 'o');

                    // add a new complete path
                    full_paths.push(path.clone());

                    path.pop();
                    robot.mov_back(&dir);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    visited.remove(&pos);
}

fn build_maze(input: &str) -> (Maze, FullPaths) {
    let mut robot = MazeRobot {
        intcode: Intcode::new_from_str(input),
    };
    let mut maze = HashMap::new();
    let mut visited = HashSet::new();
    let mut path = vec![];
    let mut full_paths = vec![];
    let init_pos = Coords { x: 0, y: 0 };

    try_moves(
        init_pos,
        &mut robot,
        &mut maze,
        &mut visited,
        &mut path,
        &mut full_paths,
    );
    (maze, full_paths)
}

fn flood(maze: &mut Maze) -> usize {
    let mut minutes = 0;
    while maze.values().any(|v| v == &'.') {
        minutes += 1;
        let mut flood_new = vec![];
        for (pos, v) in maze.iter() {
            if v == &'.' {
                // if a neighboring (NSWE) field has oxygen, it spreads to this field
                let adj_fields = [
                    Coords {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                    Coords {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    Coords {
                        x: pos.x,
                        y: pos.y - 1,
                    },
                    Coords {
                        x: pos.x,
                        y: pos.y + 1,
                    },
                ];
                if adj_fields
                    .iter()
                    .any(|a| maze.get(a).unwrap_or(&'#') == &'o')
                {
                    flood_new.push(*pos);
                }
            }
        }

        for n in flood_new {
            maze.insert(n, 'o');
        }
    }
    minutes
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let (_, full_paths) = build_maze(input);
        let shortest_path = full_paths.iter().map(|p| p.len()).min().unwrap();
        format!("{}", shortest_path)
    }

    fn star2(&self, input: &str) -> String {
        let (mut maze, _) = build_maze(input);
        format!("{}", flood(&mut maze))
    }
}
