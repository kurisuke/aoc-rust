use crate::day::Day;
use itertools::Itertools;

pub struct Day09 {}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let nums: Vec<_> = input
            .lines()
            .map(|l| l.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        format!("{}", find_cipher_weakness(&nums, 25))
    }

    fn star2(&self, input: &str) -> String {
        let nums: Vec<_> = input
            .lines()
            .map(|l| l.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        let target = find_cipher_weakness(&nums, 25);
        format!("{}", find_cont_set(&nums, target))
    }
}

fn find_cipher_weakness(nums: &[i64], n: usize) -> i64 {
    for i in n..nums.len() {
        let sums = get_sums(&nums[i - n..i]);
        if sums.iter().find(|&&v| v == nums[i]).is_none() {
            return nums[i];
        }
    }
    -1
}

fn find_cont_set(nums: &[i64], target: i64) -> i64 {
    for i in 0..nums.len() {
        let mut sum = nums[i];
        let mut j = 1;
        let mut found = false;

        while sum < target && i + j < nums.len() {
            sum += nums[i + j];
            if sum == target {
                found = true;
                break;
            }
            j += 1;
        }

        if found {
            let window = &nums[i..(i + j + 1)];
            return window.iter().min().unwrap() + window.iter().max().unwrap();
        }
    }
    -1
}

fn get_sums(window: &[i64]) -> Vec<i64> {
    window
        .iter()
        .combinations(2)
        .map(|c| c.iter().copied().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
        let nums: Vec<_> = input
            .lines()
            .map(|l| l.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        assert_eq!(find_cipher_weakness(&nums, 5), 127);
        assert_eq!(find_cont_set(&nums, 127), 62);
    }
}
