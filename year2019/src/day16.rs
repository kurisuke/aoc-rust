use common::day::Day;

pub struct Day16 {}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn coeff(x: i64, k: i64) -> i64 {
    ((x + k + 2) / (k + 1) % 4 - 1) % 2
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let mut nums = parse_input(input);

        for _ in 0..100 {
            for k in 0..nums.len() {
                let mut tmp = 0;
                #[allow(clippy::needless_range_loop)]
                for x in k..nums.len() {
                    tmp += coeff(x as i64, k as i64) * nums[x];
                }
                nums[k] = tmp.abs() % 10;
            }
        }
        nums[0..8].iter().fold(0, |a, n| a * 10 + n).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let nums = parse_input(input);
        let total_length = nums.len() * 10000; // sums will repeat worst after 10 * period

        let offset = nums[0..7].iter().fold(0, |a, n| a * 10 + n) as usize;

        // initial nums
        let mut long_nums = vec![];
        for i in (offset..total_length).rev() {
            long_nums.push(nums[i % nums.len()]);
        }

        for _ in 0..100 {
            let mut acc = 0;
            let mut tmp = vec![];
            for k in long_nums {
                acc = (acc + k) % 10;
                tmp.push(acc);
            }
            long_nums = tmp;
        }

        long_nums
            .iter()
            .rev()
            .take(8)
            .fold(0, |a, n| a * 10 + n)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day16 {};
        assert_eq!(d.star1("80871224585914546619083218645595"), "24176176");
        assert_eq!(d.star1("19617804207202209144916044189917"), "73745418");
        assert_eq!(d.star1("69317163492948606335995924319873"), "52432133");
    }

    #[test]
    fn star2() {
        let d = Day16 {};
        assert_eq!(d.star2("03036732577212944063491565474664"), "84462026");
        assert_eq!(d.star2("02935109699940807407585447034323"), "78725270");
        assert_eq!(d.star2("03081770884921959731165446850517"), "53553731");
    }
}
