use crate::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let mut ops: Vec<_> = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();
        let mut pc = 0;
        let mut steps = 0;
        while pc >= 0 && pc < ops.len() as isize {
            steps += 1;
            let pc_old = pc;
            pc += ops[pc as usize];
            ops[pc_old as usize] += 1;
        }
        format!("{}", steps)
    }

    fn star2(&self, input: &str) -> String {
        let mut ops: Vec<_> = input
            .lines()
            .map(|line| line.parse::<isize>().unwrap())
            .collect();
        let mut pc = 0;
        let mut steps = 0;
        while pc >= 0 && pc < ops.len() as isize {
            steps += 1;
            let pc_old = pc;
            pc += ops[pc as usize];
            ops[pc_old as usize] += if ops[pc_old as usize] < 3 { 1 } else { -1 };
        }
        format!("{}", steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day05 {};
        let input = r#"0
3
0
1
-3"#;
        assert_eq!(d.star1(input), "5");
        assert_eq!(d.star2(input), "10");
    }
}
