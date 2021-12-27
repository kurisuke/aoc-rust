use common::day::Day;
use util::intcode::{IntSize, Intcode};

pub struct Day21 {}

const JS_PART1: &str = r#"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
"#;

const JS_PART2: &str = r#"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
AND E T
OR H T
AND T J
RUN
"#;

fn run(input: &str, js_prog: &str) -> IntSize {
    let mut intcode = Intcode::new_from_str(input);
    intcode.write_inp_ascii(js_prog);

    intcode.run();

    let o = intcode.read_outp_all();
    *o.last().unwrap()
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        format!("{}", run(input, JS_PART1))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", run(input, JS_PART2))
    }
}
