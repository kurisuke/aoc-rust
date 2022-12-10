use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day10 {}

#[derive(Copy, Clone)]
enum Instr {
    NoOp,
    AddX(i64),
}

struct Device {
    reg: i64,
    program: Vec<Instr>,

    pc: usize,
    cycles_remain: usize,
    current_instr: Instr,
}

impl Device {
    fn new(program: Vec<Instr>) -> Device {
        Device {
            reg: 1,
            program,

            pc: 0,
            cycles_remain: 1,
            current_instr: Instr::NoOp,
        }
    }

    fn cycle(&mut self) -> i64 {
        self.cycles_remain -= 1;
        if self.cycles_remain == 0 {
            // apply result of instruction
            match self.current_instr {
                Instr::NoOp => {}
                Instr::AddX(v) => {
                    self.reg += v;
                }
            }

            // fetch next instruction and reset cycle remain counter
            self.current_instr = self.program[self.pc];
            self.cycles_remain = match self.current_instr {
                Instr::NoOp => 1,
                Instr::AddX(_) => 2,
            };
            // increase pc
            self.pc += 1;
        }

        self.reg
    }
}

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split_whitespace().collect();
            match tokens[0] {
                "noop" => Instr::NoOp,
                "addx" => Instr::AddX(tokens[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut device = Device::new(program);

        let mut sum_strengths = 0;

        for pos in 1..=220 {
            let output = device.cycle();
            if pos % 40 == 20 {
                sum_strengths += pos * output;
            }
        }

        format!("{}", sum_strengths)
    }

    fn star2(&self, input: &str) -> String {
        let program = parse_input(input);
        let mut device = Device::new(program);

        let crt_width = 40;
        let crt_height = 6;

        let mut crt = Grid2D::with_default(
            Coords {
                x: crt_width,
                y: crt_height,
            },
            &' ',
        );
        for cycle in 0..crt_width * crt_height {
            let x = cycle % crt_width;
            let y = cycle / crt_width;

            let sprite_pos = device.cycle();
            let sprite_offset = (sprite_pos - x).unsigned_abs();
            if sprite_offset <= 1 {
                crt.set(&Coords { x, y }, '█');
            }
        }
        format!("{}", crt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let d = Day10 {};
        assert_eq!(d.star1(input), "13140");

        let result_pt2 = r#"██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     "#;
        assert_eq!(d.star2(input), result_pt2);
    }
}
