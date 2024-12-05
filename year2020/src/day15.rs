use common::day::Day;

pub struct Day15 {}

fn find_num(start_nums: &[u32], n: u32) -> u32 {
    let mut prev_turn = vec![0; n as usize];

    for (i, n) in start_nums.iter().enumerate().take(start_nums.len() - 1) {
        prev_turn[*n as usize] = i as u32 + 1;
    }
    let mut last_spoken = *start_nums.last().unwrap();

    for turn in (start_nums.len() as u32)..n {
        let next_spoken = if prev_turn[last_spoken as usize] == 0 {
            0
        } else {
            turn - prev_turn[last_spoken as usize]
        };

        prev_turn[last_spoken as usize] = turn;
        last_spoken = next_spoken;
    }

    last_spoken
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        format!("{}", find_num(&start_nums, 2020))
    }

    fn star2(&self, input: &str) -> String {
        let start_nums: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
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
