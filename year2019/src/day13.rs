use common::day::Day;
use std::cmp::Ordering;
use std::collections::HashMap;
use util::intcode::{IntSize, Intcode, RunState};

pub struct Day13 {}

type Screen = HashMap<(IntSize, IntSize), IntSize>;

fn read_screen(screen: &mut Screen, intcode: &mut Intcode) -> IntSize {
    let mut score = 0;
    intcode.run();
    while let Some(x) = intcode.read_outp() {
        let y = intcode.read_outp().unwrap();
        let id = intcode.read_outp().unwrap();
        if x == -1 && y == 0 {
            score = id;
        } else {
            screen.insert((x, y), id);
        }
    }
    score
}

fn play(intcode: &mut Intcode) -> IntSize {
    let mut screen = HashMap::new();
    let mut score = 0;

    while intcode.state != RunState::Halted {
        score = read_screen(&mut screen, intcode);
        let paddle_x = screen
            .iter()
            .find(|(_, v)| v == &&3)
            .map(|(k, _)| k.0)
            .unwrap();
        let ball_x = screen
            .iter()
            .find(|(_, v)| v == &&4)
            .map(|(k, _)| k.0)
            .unwrap();

        let inp = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };

        intcode.write_inp(inp);
    }
    score
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        let mut screen = HashMap::new();
        let _ = read_screen(&mut screen, &mut intcode);
        let num_blocks = screen.values().filter(|id| id == &&2).count();
        format!("{}", num_blocks)
    }

    fn star2(&self, input: &str) -> String {
        let mut intcode = Intcode::new_from_str(input);
        intcode.set_mem_at(0, 2);
        format!("{}", play(&mut intcode))
    }
}
