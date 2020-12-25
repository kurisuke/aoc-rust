use crate::day::Day;

pub struct Day25 {}

fn get_loop_size(target_key: usize, subject: usize, div: usize) -> usize {
    let mut key = 1;
    let mut loops = 0;
    while key != target_key {
        key = (subject * key) % div;
        loops += 1;
    }
    loops
}

fn perform_loops(n: usize, subject: usize, div: usize) -> usize {
    let mut key = 1;
    for _ in 0..n {
        key = (subject * key) % div;
    }
    key
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let mut it = input.lines();
        let card_key = it.next().unwrap().parse::<usize>().unwrap();
        let door_key = it.next().unwrap().parse::<usize>().unwrap();
        let card_loops = get_loop_size(card_key, 7, 20201227);
        let encr_key = perform_loops(card_loops, door_key, 20201227);
        format!("{}", encr_key)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "5764801
17807724";
        let d = Day25 {};
        assert_eq!(d.star1(input), "14897079");
    }
}
