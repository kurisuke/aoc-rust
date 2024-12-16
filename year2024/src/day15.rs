use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day15 {}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let (mut grid, dirs) = parse_input(input);
        let mut cur_pos = grid.find(Field::Robot).unwrap();
        for dir in dirs {
            cur_pos = move_robot(&mut grid, cur_pos, dir);
        }
        gps_sum(&grid).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[derive(PartialEq, Eq)]
enum Field {
    Robot,
    Box,
    Wall,
    Empty,
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

fn move_robot(grid: &mut Grid2D<Field>, mut cur_pos: Coords, dir: Direction) -> Coords {
    let new_pos = cur_pos.mov(dir);
    match grid.at(&new_pos).unwrap() {
        Field::Robot => unreachable!(),
        Field::Box => {
            let mut shift_pos = new_pos.mov(dir);
            let can_move = loop {
                match grid.at(&shift_pos).unwrap() {
                    Field::Robot => unreachable!(),
                    Field::Box => {}
                    Field::Wall => {
                        break false;
                    }
                    Field::Empty => {
                        break true;
                    }
                }
                shift_pos = shift_pos.mov(dir);
            };

            if can_move {
                grid.set(&shift_pos, Field::Box);
                grid.set(&new_pos, Field::Robot);
                grid.set(&cur_pos, Field::Empty);
                cur_pos = new_pos;
            }
        }
        Field::Wall => {
            // cannot move
        }
        Field::Empty => {
            grid.set(&new_pos, Field::Robot);
            grid.set(&cur_pos, Field::Empty);
            cur_pos = new_pos;
        }
    }

    cur_pos
}

fn gps_sum(grid: &Grid2D<Field>) -> i64 {
    grid.filter(&[Field::Box])
        .iter()
        .map(|c| c.x + 100 * c.y)
        .sum()
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
}
