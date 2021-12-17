use common::day::Day;
use util::wrist::Wrist;

pub struct Day19 {}

fn factors(n: usize) -> Vec<usize> {
    let mut fs = vec![];
    let upper = (f64::from(n as i32).sqrt() + 1.0) as usize;
    for i in 1..=upper {
        if n % i == 0 {
            fs.push(i);
            if i * i != n {
                fs.push(n / i);
            }
        }
    }
    fs
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let mut wrist = Wrist::from_str(input);
        wrist.run();
        format!("{}", wrist.reg[0])
    }

    fn star2(&self, input: &str) -> String {
        let mut wrist = Wrist::from_str(input);
        wrist.reg[0] = 1;
        let max_reg = wrist.run_y2018_s2(30);
        let sum_factors = factors(max_reg).into_iter().sum::<usize>();
        format!("{}", sum_factors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day19 {};

        let input = r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#;

        assert_eq!(d.star1(input), "6");
    }
}
