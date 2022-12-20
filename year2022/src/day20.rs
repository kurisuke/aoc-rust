use common::day::Day;

use std::collections::VecDeque;

pub struct Day20 {}

struct NumberList(VecDeque<(usize, isize)>);

impl NumberList {
    fn new(nums: &[isize], mul: isize) -> NumberList {
        NumberList(
            nums.iter()
                .enumerate()
                .map(|(orig_pos, shift)| (orig_pos, *shift * mul))
                .collect(),
        )
    }

    fn nth_after_zero(&self, n: usize) -> isize {
        let src_zero = self.0.iter().position(|(_, val)| *val == 0).unwrap();
        self.0[(src_zero + n) % self.0.len()].1
    }

    fn mov_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for i in 0..self.0.len() {
                self.mov(i);
            }
        }
    }

    fn mov(&mut self, pos: usize) {
        let src_pos = self
            .0
            .iter()
            .position(|(orig_pos, _)| *orig_pos == pos)
            .unwrap();

        // rotate move element to front
        self.0.rotate_left(src_pos);
        // pop off and rotate to insertion point
        let tmp = self.0.pop_front().unwrap();
        let shift = tmp.1.rem_euclid(self.0.len() as isize) as usize;
        self.0.rotate_left(shift);
        // insert back in between target elements
        self.0.push_back(tmp);
    }
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let nums: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut nums = NumberList::new(&nums, 1);
        nums.mov_rounds(1);
        format!(
            "{}",
            nums.nth_after_zero(1000) + nums.nth_after_zero(2000) + nums.nth_after_zero(3000)
        )
    }

    fn star2(&self, input: &str) -> String {
        let nums: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut nums = NumberList::new(&nums, 811589153);
        nums.mov_rounds(10);
        format!(
            "{}",
            nums.nth_after_zero(1000) + nums.nth_after_zero(2000) + nums.nth_after_zero(3000)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"1
2
-3
3
-2
0
4"#;
        let d = Day20 {};
        assert_eq!(d.star1(input), "3");
        assert_eq!(d.star2(input), "1623178306");
    }
}
