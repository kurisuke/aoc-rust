use common::day::Day;

pub struct Day06 {}

fn parse_input(input: &str) -> [u64; 9] {
    let mut fish = [0; 9];
    for n in input.trim().split(',') {
        fish[n.parse::<usize>().unwrap()] += 1;
    }
    fish
}

fn evolve(fish: &mut [u64; 9]) {
    let new_fish = fish[0];
    fish[0] = fish[1];
    fish[1] = fish[2];
    fish[2] = fish[3];
    fish[3] = fish[4];
    fish[4] = fish[5];
    fish[5] = fish[6];
    fish[6] = fish[7] + new_fish;
    fish[7] = fish[8];
    fish[8] = new_fish;
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let mut fish = parse_input(input);
        for _ in 0..80 {
            evolve(&mut fish);
        }
        format!("{}", fish.iter().sum::<u64>())
    }

    fn star2(&self, input: &str) -> String {
        let mut fish = parse_input(input);
        for _ in 0..256 {
            evolve(&mut fish);
        }
        format!("{}", fish.iter().sum::<u64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "3,4,3,1,2";
        let d = Day06 {};
        assert_eq!(d.star1(input), "5934");
        assert_eq!(d.star2(input), "26984457539");
    }
}
