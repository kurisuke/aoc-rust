use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let (start_coords, grid) = parse_input(input);
        traverse(start_coords, &grid).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe::NS),
            '-' => Some(Pipe::EW),
            'L' => Some(Pipe::NE),
            'J' => Some(Pipe::NW),
            '7' => Some(Pipe::SW),
            'F' => Some(Pipe::SE),
            '.' => None,
            _ => unreachable!(),
        }
    }

    fn has_connector_at(&self, direction: &Direction) -> bool {
        match direction {
            Direction::N => matches!(self, Pipe::NS | Pipe::NE | Pipe::NW),
            Direction::E => matches!(self, Pipe::EW | Pipe::NE | Pipe::SE),
            Direction::S => matches!(self, Pipe::NS | Pipe::SW | Pipe::SE),
            Direction::W => matches!(self, Pipe::EW | Pipe::NW | Pipe::SW),
            _ => unreachable!(),
        }
    }
}

fn start_shape(grid: &Grid2D<char>) -> Option<Pipe> {
    let start = grid.find('S').unwrap();
    let connects_to_north = grid
        .at(&start.mov(Direction::N))
        .is_some_and(|c| Pipe::from_char(*c).is_some_and(|p| p.has_connector_at(&Direction::S)));
    let connects_to_south = grid
        .at(&start.mov(Direction::S))
        .is_some_and(|c| Pipe::from_char(*c).is_some_and(|p| p.has_connector_at(&Direction::N)));
    let connects_to_east = grid
        .at(&start.mov(Direction::E))
        .is_some_and(|c| Pipe::from_char(*c).is_some_and(|p| p.has_connector_at(&Direction::W)));
    let connects_to_west = grid
        .at(&start.mov(Direction::W))
        .is_some_and(|c| Pipe::from_char(*c).is_some_and(|p| p.has_connector_at(&Direction::E)));

    if connects_to_north && connects_to_south {
        Some(Pipe::NS)
    } else if connects_to_east && connects_to_west {
        Some(Pipe::EW)
    } else if connects_to_north && connects_to_east {
        Some(Pipe::NE)
    } else if connects_to_north && connects_to_west {
        Some(Pipe::NW)
    } else if connects_to_south && connects_to_west {
        Some(Pipe::SW)
    } else if connects_to_south && connects_to_east {
        Some(Pipe::SE)
    } else {
        None
    }
}

fn parse_input(input: &str) -> (Coords, Grid2D<Option<Pipe>>) {
    let grid = Grid2D::new(input).unwrap();
    let start_coords = grid.find('S').unwrap();
    let start_shape = start_shape(&grid).unwrap();

    let grid = Grid2D::new_by(input, |c| match c {
        'S' => Some(start_shape),
        _ => Pipe::from_char(c),
    })
    .unwrap();

    (start_coords, grid)
}

fn mov(grid: &Grid2D<Option<Pipe>>, pos: Coords, d: Direction) -> (Coords, Direction) {
    let pos_new = pos.mov(d);
    let pipe_new = grid.at(&pos_new).unwrap().unwrap();

    let d_new = match (pipe_new, d) {
        (Pipe::NS, Direction::N) => Direction::N,
        (Pipe::NS, Direction::S) => Direction::S,
        (Pipe::EW, Direction::E) => Direction::E,
        (Pipe::EW, Direction::W) => Direction::W,
        (Pipe::NE, Direction::S) => Direction::E,
        (Pipe::NE, Direction::W) => Direction::N,
        (Pipe::SE, Direction::N) => Direction::E,
        (Pipe::SE, Direction::W) => Direction::S,
        (Pipe::NW, Direction::S) => Direction::W,
        (Pipe::NW, Direction::E) => Direction::N,
        (Pipe::SW, Direction::N) => Direction::W,
        (Pipe::SW, Direction::E) => Direction::S,
        _ => unreachable!(),
    };
    (pos_new, d_new)
}

fn traverse(start_coords: Coords, grid: &Grid2D<Option<Pipe>>) -> usize {
    let mut mov_counter = 0;
    let mut mov1_pos = start_coords;
    let mut mov2_pos = start_coords;
    let (mut mov1_dir, mut mov2_dir) = match grid.at(&start_coords).unwrap().unwrap() {
        Pipe::NS => (Direction::N, Direction::S),
        Pipe::EW => (Direction::E, Direction::W),
        Pipe::NE => (Direction::N, Direction::E),
        Pipe::NW => (Direction::N, Direction::W),
        Pipe::SW => (Direction::S, Direction::W),
        Pipe::SE => (Direction::S, Direction::E),
    };

    loop {
        mov_counter += 1;
        (mov1_pos, mov1_dir) = mov(grid, mov1_pos, mov1_dir);
        (mov2_pos, mov2_dir) = mov(grid, mov2_pos, mov2_dir);
        if mov1_pos == mov2_pos {
            break;
        }
    }

    mov_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;

        let d = Day10 {};
        assert_eq!(d.star1(input), "4");
    }

    #[test]
    fn ex2() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        let d = Day10 {};
        assert_eq!(d.star1(input), "8");
    }

    #[test]
    fn test_start_shape() {
        let input = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

        let grid = Grid2D::new(input).unwrap();
        assert_eq!(start_shape(&grid), Some(Pipe::SE));

        let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

        let grid = Grid2D::new(input).unwrap();
        assert_eq!(start_shape(&grid), Some(Pipe::SE));
    }
}
