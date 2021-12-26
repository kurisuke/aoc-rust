use common::day::Day;
use util::intcode::{IntSize, Intcode};

pub struct Day19 {}

fn is_affected(mut intcode: Intcode, x: IntSize, y: IntSize) -> bool {
    intcode.write_inp(x);
    intcode.write_inp(y);
    intcode.run();
    intcode.read_outp().unwrap() == 1
}

fn can_fit(intcode: Intcode, x: IntSize, y: IntSize) -> bool {
    is_affected(intcode.clone(), x, y)
        && is_affected(intcode.clone(), x + 99, y)
        && is_affected(intcode.clone(), x, y + 99)
        && is_affected(intcode, x + 99, y + 99)
}

fn dist(p: &(IntSize, IntSize)) -> IntSize {
    p.0 * p.0 + p.1 * p.1
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let intcode = Intcode::new_from_str(input);
        let mut num_affected = 0;
        for x in 0..50 {
            for y in 0..50 {
                if is_affected(intcode.clone(), x, y) {
                    num_affected += 1;
                }
            }
        }
        format!("{}", num_affected)
    }

    fn star2(&self, input: &str) -> String {
        let intcode = Intcode::new_from_str(input);

        let mut fit_list = vec![];
        let mut w = 256;
        let (mut x_min, mut x_max) = (0, 2048);
        let (mut y_min, mut y_max) = (0, 2048);
        let mut min_point = (-1, -1);

        while w >= 1 {
            for x in (x_min..=x_max).step_by(w) {
                for y in (y_min..=y_max).step_by(w) {
                    if can_fit(intcode.clone(), x, y) {
                        fit_list.push((x, y));
                    }
                }
            }
            min_point = *fit_list
                .iter()
                .min_by(|a, b| dist(a).cmp(&dist(b)))
                .unwrap();
            x_min = min_point.0 - (4 * w) as i64;
            x_max = min_point.0;
            y_min = min_point.1 - (4 * w) as i64;
            y_max = min_point.1;

            w /= 4;
        }
        let ret = min_point.0 * 10000 + min_point.1;
        format!("{}", ret)
    }
}
