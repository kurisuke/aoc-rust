use crate::day::Day;

pub struct Day15 {}

fn find_num(start_nums: &[usize], n: usize) -> usize {
    let mut prev_turn = vec![0; n];

    for (i, n) in start_nums.iter().enumerate().take(start_nums.len() - 1) {
        prev_turn[*n] = i + 1;
    }
    let mut last_spoken = *start_nums.last().unwrap();

    for turn in start_nums.len()..n {
        let next_spoken = if prev_turn[last_spoken] == 0 {
            0
        } else {
            turn - prev_turn[last_spoken]
        };

        prev_turn[last_spoken] = turn;

        // next turn
        last_spoken = next_spoken;
    }

    last_spoken
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        format!("{}", find_num(&start_nums, 2020))
    }

    fn star2(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        format!("{}", find_num(&start_nums, 30000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day15 {};
        assert_eq!(d.star1("0,3,6"), "436");
        assert_eq!(d.star1("1,3,2"), "1");
        assert_eq!(d.star1("2,1,3"), "10");
        assert_eq!(d.star1("1,2,3"), "27");
        assert_eq!(d.star1("2,3,1"), "78");
        assert_eq!(d.star1("3,2,1"), "438");
        assert_eq!(d.star1("3,1,2"), "1836");
    }
}
