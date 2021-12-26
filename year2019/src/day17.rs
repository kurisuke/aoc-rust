use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};
use util::intcode::{IntSize, Intcode};

pub struct Day17 {}

fn parse_input(input: &str) -> Grid2D<char> {
    let mut intcode = Intcode::new_from_str(input);
    intcode.run();
    let out_str = intcode.read_outp_ascii();
    Grid2D::new(out_str.trim()).unwrap()
}

fn alignment_params(grid: &Grid2D<char>) -> i64 {
    let mut acc = 0;
    for c in grid.coords_iter() {
        if grid.at(&c).unwrap() == &'#'
            && grid
                .neighbors_cardinal(&c)
                .into_iter()
                .all(|n| n.unwrap_or(&'.') == &'#')
        {
            acc += c.x * c.y
        }
    }
    acc
}

fn get_movements(grid: &mut Grid2D<char>) -> Vec<String> {
    let mut cmd_list = vec![];
    while grid.find('#').is_some() {
        if let Some(m) = next_move(grid) {
            cmd_list.push(m);
        }
    }

    cmd_list
}

fn next_move(grid: &mut Grid2D<char>) -> Option<String> {
    let steps = do_move(grid);
    if steps > 0 {
        Some(steps.to_string())
    } else {
        do_turn(grid)
    }
}

fn do_move(grid: &mut Grid2D<char>) -> usize {
    let (mut pos_new, orient) = find_robot(grid).unwrap();

    // check if move in current direction is possible
    let mut steps = 0;
    loop {
        let pos_last = pos_new;
        pos_new = match orient {
            '>' => pos_new.mov(Direction::E),
            '<' => pos_new.mov(Direction::W),
            '^' => pos_new.mov(Direction::N),
            'v' => pos_new.mov(Direction::S),
            _ => unreachable!(),
        };
        if grid.at(&pos_new).is_some()
            && (grid.at(&pos_new).unwrap() == &'#' || grid.at(&pos_new).unwrap() == &'X')
        {
            // its also okay to travel over already visited fields
            // move possible, add step
            steps += 1;
            grid.set(&pos_last, 'X');
            grid.set(&pos_new, orient);
        } else {
            // moving no longer possible
            break;
        }
    }
    steps
}

fn do_turn(grid: &mut Grid2D<char>) -> Option<String> {
    let (pos_last, orient) = find_robot(grid).unwrap();

    let dirs = ['<', '^', '>', 'v'];

    for dir_off in [1, 3] {
        let cur_dir_idx = dirs.iter().position(|&x| x == orient).unwrap();
        let new_orient = dirs[(cur_dir_idx + dir_off) % dirs.len()];
        let pos_new = match new_orient {
            '>' => pos_last.mov(Direction::E),
            '<' => pos_last.mov(Direction::W),
            '^' => pos_last.mov(Direction::N),
            'v' => pos_last.mov(Direction::S),
            _ => unreachable!(),
        };
        if grid.at(&pos_new).is_some() && grid.at(&pos_new).unwrap() == &'#' {
            grid.set(&pos_last, new_orient);
            return match dir_off {
                1 => Some(String::from("R")),
                3 => Some(String::from("L")),
                _ => unreachable!(),
            };
        }
    }
    None
}

fn find_robot(grid: &Grid2D<char>) -> Option<(Coords, char)> {
    for c in grid.coords_iter() {
        let orient = grid.at(&c).unwrap();
        if orient == &'<' || orient == &'>' || orient == &'^' || orient == &'v' {
            return Some((c, *orient));
        }
    }
    None
}

fn part2(program: &str, grid: &mut Grid2D<char>) -> IntSize {
    let movements = get_movements(grid);

    // manual partitioning for the input, as I could not be bothered
    // to code a general solution
    let main = "A,B,B,C,B,C,B,C,A,A";
    let prog_a = "L,6,R,8,L,4,R,8,L,12";
    let prog_b = "L,12,R,10,L,4";
    let prog_c = "L,12,L,6,L,4,L,4";
    let expansion = main
        .replace("A", prog_a)
        .replace("B", prog_b)
        .replace("C", prog_c);
    assert_eq!(expansion, movements.join(","));

    let mut intcode = Intcode::new_from_str(program);
    intcode.set_mem_at(0, 2);
    feed_program(&mut intcode, main);
    feed_program(&mut intcode, prog_a);
    feed_program(&mut intcode, prog_b);
    feed_program(&mut intcode, prog_c);

    // disable continuous video feed
    intcode.write_inp_ascii("n\n");
    intcode.run();

    let outputs = intcode.read_outp_all();
    *outputs.last().unwrap()
}

fn feed_program(intcode: &mut Intcode, program: &str) {
    intcode.write_inp_ascii(program);
    intcode.write_inp_ascii("\n");
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        format!("{}", alignment_params(&grid))
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = parse_input(input);
        format!("{}", part2(input, &mut grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."#;

        let grid = Grid2D::new(input).unwrap();
        assert_eq!(alignment_params(&grid), 76);
    }
}
