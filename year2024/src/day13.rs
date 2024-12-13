use common::day::Day;
use regex::Regex;

pub struct Day13 {}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let machines = parse_input(input);
        let tokens = machines.iter().map(|m| m.tokens()).sum::<isize>();
        tokens.to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut machines = parse_input(input);
        for machine in machines.iter_mut() {
            machine.prize_part2();
        }
        let tokens = machines.iter().map(|m| m.tokens()).sum::<isize>();
        tokens.to_string()
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

type Matrix2x2 = ((isize, isize), (isize, isize));

fn det(m: &Matrix2x2) -> isize {
    m.0 .0 * m.1 .1 - m.0 .1 * m.1 .0
}

impl Machine {
    fn tokens(&self) -> isize {
        let a = (
            (self.button_a.x, self.button_b.x),
            (self.button_a.y, self.button_b.y),
        );
        let det_a = det(&a);
        if det_a == 0 {
            return 0;
        }

        let a1 = (
            (self.prize.x, self.button_b.x),
            (self.prize.y, self.button_b.y),
        );
        let det_a1 = det(&a1);
        let a2 = (
            (self.button_a.x, self.prize.x),
            (self.button_a.y, self.prize.y),
        );
        let det_a2 = det(&a2);

        if det_a1 % det_a == 0 && det_a2 % det_a == 0 {
            let x1 = det_a1 / det_a;
            let x2 = det_a2 / det_a;
            if x1 > 0 && x2 > 0 {
                x1 * 3 + x2
            } else {
                0
            }
        } else {
            0
        }
    }

    fn prize_part2(&mut self) {
        self.prize.x += 10000000000000;
        self.prize.y += 10000000000000;
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let re_button = Regex::new(r"^Button \w: X\+(\d+), Y\+(\d+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut machines = vec![];
    for machine in input.split("\n\n") {
        let mut it = machine.lines();
        let caps = re_button.captures(it.next().unwrap()).unwrap();
        let a = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        let caps = re_button.captures(it.next().unwrap()).unwrap();
        let b = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        let caps = re_prize.captures(it.next().unwrap()).unwrap();
        let prize = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        machines.push(Machine {
            button_a: a,
            button_b: b,
            prize,
        });
    }

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn ex1() {
        let d = Day13 {};
        assert_eq!(d.star1(INPUT), "480");
    }
}
