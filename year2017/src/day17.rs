use common::day::Day;

pub struct Day17 {}

fn fill_buffer(times: usize, step: usize) -> usize {
    let mut buf = vec![0; times + 1];
    let mut cur = 0;
    for i in 1..=times {
        for _ in 0..=step {
            cur = buf[cur];
        }

        let after = buf[cur];
        buf[cur] = i;
        buf[i] = after;
    }
    buf[times]
}

fn after_first(times: usize, step: usize) -> usize {
    let mut cur = 0;
    let mut after_first = 0;
    for i in 1..=times {
        cur = (cur + step + 1) % i;
        if cur == 0 {
            after_first = i;
        }
    }
    after_first
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        format!(
            "{}",
            fill_buffer(2017, input.trim().parse::<usize>().unwrap())
        )
    }

    fn star2(&self, input: &str) -> String {
        format!(
            "{}",
            after_first(50_000_000, input.trim().parse::<usize>().unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day17 {};
        assert_eq!(d.star1("3"), "638");
    }
}
