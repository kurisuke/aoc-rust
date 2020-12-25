use crate::day::Day;
use crate::util::grid2d::{Coords, Grid2D};

pub struct Day06 {}

enum Cmd {
    On,
    Off,
    Toggle,
}

struct Instruction {
    cmd: Cmd,
    start: Coords,
    stop: Coords,
}

fn perform_star1(grid: &mut Grid2D<bool>, instr: &Instruction) {
    for x in instr.start.x..=instr.stop.x {
        for y in instr.start.y..=instr.stop.y {
            let c = Coords { x, y };
            match instr.cmd {
                Cmd::On => {
                    grid.set(&c, true);
                }
                Cmd::Off => {
                    grid.set(&c, false);
                }
                Cmd::Toggle => {
                    grid.set(&c, !*grid.at(&c).unwrap());
                }
            }
        }
    }
}

fn perform_star2(grid: &mut Grid2D<u32>, instr: &Instruction) {
    for x in instr.start.x..=instr.stop.x {
        for y in instr.start.y..=instr.stop.y {
            let c = Coords { x, y };
            let v = *grid.at(&c).unwrap();
            match instr.cmd {
                Cmd::On => {
                    grid.set(&c, v + 1);
                }
                Cmd::Off => {
                    grid.set(&c, if v > 0 { v - 1 } else { 0 });
                }
                Cmd::Toggle => {
                    grid.set(&c, v + 2);
                }
            }
        }
    }
}

fn parse_coords(s: &str) -> (Coords, Coords) {
    let mut it = s.split(" through ");
    let start_str = it.next().unwrap();
    let coords_start: Vec<_> = start_str
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let stop_str = it.next().unwrap();
    let coords_stop: Vec<_> = stop_str
        .split(',')
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    (
        Coords {
            x: coords_start[0],
            y: coords_start[1],
        },
        Coords {
            x: coords_stop[0],
            y: coords_stop[1],
        },
    )
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instr = vec![];
    for line in input.lines() {
        if line.starts_with("turn on") {
            let (start, stop) = parse_coords(&line[8..]);
            instr.push(Instruction {
                cmd: Cmd::On,
                start,
                stop,
            });
        } else if line.starts_with("turn off") {
            let (start, stop) = parse_coords(&line[9..]);
            instr.push(Instruction {
                cmd: Cmd::Off,
                start,
                stop,
            });
        } else if line.starts_with("toggle") {
            let (start, stop) = parse_coords(&line[7..]);
            instr.push(Instruction {
                cmd: Cmd::Toggle,
                start,
                stop,
            });
        }
    }
    instr
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let instrs = parse_input(input);
        let mut grid = Grid2D::with_default(Coords { x: 1000, y: 1000 }, &false);
        for instr in instrs.iter() {
            perform_star1(&mut grid, instr);
        }
        format!("{}", grid.count(true))
    }

    fn star2(&self, input: &str) -> String {
        let instrs = parse_input(input);
        let mut grid = Grid2D::with_default(Coords { x: 1000, y: 1000 }, &0u32);
        for instr in instrs.iter() {
            perform_star2(&mut grid, instr);
        }
        format!("{}", grid.iter().sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day06 {};
        assert_eq!(d.star1("turn on 0,0 through 999,999"), "1000000");
        assert_eq!(d.star1("toggle 0,0 through 999,0"), "1000");
        assert_eq!(d.star1("turn off 499,499 through 500,500"), "0");
    }

    #[test]
    fn star2() {
        let d = Day06 {};
        assert_eq!(d.star2("turn on 0,0 through 0,0"), "1");
        assert_eq!(d.star2("toggle 0,0 through 999,999"), "2000000");
    }
}
