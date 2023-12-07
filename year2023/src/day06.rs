use common::day::Day;

pub struct Day06 {}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let races = parse_part1(input);
        races
            .into_iter()
            .map(|race| race.ways_to_win())
            .product::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let race = parse_part2(input);
        race.ways_to_win().to_string()
    }
}

struct Race {
    time: usize,
    distance: usize,
}

fn parse_part1(input: &str) -> Vec<Race> {
    let lines: Vec<_> = input.lines().collect();
    let times: Vec<_> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let distances: Vec<_> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_part2(input: &str) -> Race {
    let lines: Vec<_> = input.lines().collect();
    let time: String = lines[0].split_whitespace().skip(1).collect();
    let distance: String = lines[1].split_whitespace().skip(1).collect();
    Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    }
}

impl Race {
    fn ways_to_win(&self) -> usize {
        (1..self.time)
            .filter(|n| (self.time - n) * n > self.distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn star1() {
        let d = Day06 {};
        assert_eq!(d.star1(INPUT), "288");
    }

    #[test]
    fn star2() {
        let d = Day06 {};
        assert_eq!(d.star2(INPUT), "71503");
    }
}
