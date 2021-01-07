use common::day::Day;

pub struct Day19 {}

#[allow(dead_code)]
fn run_part1(n: usize) -> usize {
    let mut elves: Vec<_> = (0..n).map(|i| (i + 1) % n).collect();
    let mut cur = 0;
    while elves[cur] != cur {
        // remove the next one from the circle
        elves[cur] = elves[elves[cur]];
        cur = elves[cur];
    }
    cur + 1
}

#[allow(dead_code)]
fn run_part2(n: usize) -> usize {
    let mut elves: Vec<_> = (0..n).map(|i| (i + 1) % n).collect();
    let mut cur = 0;
    let mut circle_size = n;

    while elves[cur] != cur {
        let move_steps = circle_size / 2;
        let mut before_remove_elf = cur;
        for _ in 0..(move_steps - 1) {
            before_remove_elf = elves[before_remove_elf];
        }
        // remove the next one from the circle
        elves[before_remove_elf] = elves[elves[before_remove_elf]];
        circle_size -= 1;

        // to next elf
        cur = elves[cur];
    }
    cur + 1
}

fn calc_part2(n: usize) -> usize {
    let mut pow = 1;
    while pow * 3 < n {
        pow *= 3;
    }
    let diff_to_pow = n - pow;
    if diff_to_pow <= pow {
        diff_to_pow
    } else {
        2 * diff_to_pow - pow
    }
}

fn calc_part1(n: usize) -> usize {
    let mut pow = 1;
    while pow * 2 <= n {
        pow *= 2;
    }
    1 + (n - pow) * 2
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let num_elves = input.trim().parse::<usize>().unwrap();
        format!("{}", calc_part1(num_elves))
    }

    fn star2(&self, input: &str) -> String {
        let num_elves = input.trim().parse::<usize>().unwrap();
        format!("{}", calc_part2(num_elves))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(calc_part1(5), 3);
        assert_eq!(calc_part2(5), 2);
    }

    #[test]
    fn test_eq_run_calc_part1() {
        for i in 2..=1000 {
            let run_res = run_part1(i);
            let calc_res = calc_part1(i);
            println!("{}: {}, {}", i, run_res, calc_res);
            assert_eq!(run_res, calc_res);
        }
    }

    #[test]
    fn test_eq_run_calc_part2() {
        for i in 2..=1000 {
            let run_res = run_part2(i);
            let calc_res = calc_part2(i);
            println!("{}: {}, {}", i, run_res, calc_res);
            assert_eq!(run_res, calc_res);
        }
    }
}
