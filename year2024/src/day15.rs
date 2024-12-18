use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day15 {}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let (mut grid, dirs) = parse_input(input);
        execute_moves(&mut grid, &dirs).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let (grid, dirs) = parse_input(input);
        let mut grid = scale_wide(grid);
        execute_moves(&mut grid, &dirs).to_string()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default)]
enum Field {
    #[default]
    Empty,
    Wall,
    Robot,
    Box,
    BoxLeft,
    BoxRight,
}

fn parse_input(input: &str) -> (Grid2D<Field>, Vec<Direction>) {
    let mut secs = input.split("\n\n");
    let grid = Grid2D::new_by(secs.next().unwrap(), |c| match c {
        '@' => Field::Robot,
        'O' => Field::Box,
        '#' => Field::Wall,
        '.' => Field::Empty,
        _ => unreachable!(),
    })
    .unwrap();

    let dirs = secs
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| match c {
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            'v' => Some(Direction::S),
            '<' => Some(Direction::W),
            '\n' => None,
            _ => {
                unreachable!()
            }
        })
        .collect();

    (grid, dirs)
}

fn execute_moves(grid: &mut Grid2D<Field>, dirs: &[Direction]) -> i64 {
    let mut cur_pos = grid.find(Field::Robot).unwrap();
    for dir in dirs {
        if can_push(grid, &cur_pos, *dir) {
            cur_pos = push(grid, &cur_pos, *dir);
        }
    }
    gps_sum(grid)
}

fn can_push(grid: &Grid2D<Field>, cur_pos: &Coords, dir: Direction) -> bool {
    let new_pos = cur_pos.mov(dir);
    match grid.at(&new_pos).unwrap() {
        Field::Box => can_push(grid, &new_pos, dir),
        Field::Wall => false,
        Field::Empty => true,
        // part 2
        Field::BoxLeft => {
            if dir == Direction::N || dir == Direction::S {
                can_push(grid, &new_pos, dir) && can_push(grid, &new_pos.mov(Direction::E), dir)
            } else {
                can_push(grid, &new_pos, dir)
            }
        }
        Field::BoxRight => {
            if dir == Direction::N || dir == Direction::S {
                can_push(grid, &new_pos, dir) && can_push(grid, &new_pos.mov(Direction::W), dir)
            } else {
                can_push(grid, &new_pos, dir)
            }
        }
        _ => unreachable!(),
    }
}

fn push(grid: &mut Grid2D<Field>, cur_pos: &Coords, dir: Direction) -> Coords {
    let new_value = *grid.at(cur_pos).unwrap();
    let new_pos = cur_pos.mov(dir);
    match grid.at(&new_pos).unwrap() {
        Field::Empty => {
            // nothing to do
        }
        Field::Box => {
            // move that box first
            push(grid, &new_pos, dir);
        }
        // part 2
        Field::BoxLeft => {
            if dir == Direction::N || dir == Direction::S {
                push(grid, &new_pos, dir);
                push(grid, &new_pos.mov(Direction::E), dir);
            } else {
                push(grid, &new_pos, dir);
            }
        }
        Field::BoxRight => {
            if dir == Direction::N || dir == Direction::S {
                push(grid, &new_pos, dir);
                push(grid, &new_pos.mov(Direction::W), dir);
            } else {
                push(grid, &new_pos, dir);
            }
        }
        _ => unreachable!(),
    }
    grid.set(&new_pos, new_value);
    grid.set(cur_pos, Field::Empty);

    new_pos
}

fn gps_sum(grid: &Grid2D<Field>) -> i64 {
    grid.filter(&[Field::Box, Field::BoxLeft])
        .iter()
        .map(|c| c.x + 100 * c.y)
        .sum()
}

fn scale_wide(grid: Grid2D<Field>) -> Grid2D<Field> {
    let mut grid_new = Grid2D::with_default(
        Coords {
            x: grid.width() * 2,
            y: grid.height(),
        },
        &Field::Wall,
    );

    for (pos, value) in grid.enumerate() {
        let (left, right) = match value {
            Field::Robot => (Field::Robot, Field::Empty),
            Field::Box => (Field::BoxLeft, Field::BoxRight),
            Field::Wall => (Field::Wall, Field::Wall),
            Field::Empty => (Field::Empty, Field::Empty),
            _ => unreachable!(),
        };
        grid_new.set(
            &Coords {
                x: pos.x * 2,
                y: pos.y,
            },
            left,
        );
        grid_new.set(
            &Coords {
                x: pos.x * 2 + 1,
                y: pos.y,
            },
            right,
        );
    }

    grid_new
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const INPUT2: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn star1() {
        let d = Day15 {};
        assert_eq!(d.star1(INPUT1), "2028");
        assert_eq!(d.star1(INPUT2), "10092");
    }

    #[test]
    fn star2() {
        let d = Day15 {};
        assert_eq!(d.star2(INPUT2), "9021");
    }
}
