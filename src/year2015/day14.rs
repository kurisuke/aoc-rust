use crate::day::Day;

pub struct Day14 {}

struct ReindeerStats {
    fly_speed: u64,
    fly_length: u64,
    rest_length: u64,
}

fn distance_after(stats: &ReindeerStats, secs: u64) -> u64 {
    let cycle_length = stats.fly_length + stats.rest_length;
    let num_full_cycles = secs / cycle_length;
    let cycle_offset = secs % cycle_length;
    let last_cycle_distance = cycle_offset.min(stats.fly_length) * stats.fly_speed;
    num_full_cycles * (stats.fly_length * stats.fly_speed) + last_cycle_distance
}

fn parse_input(input: &str) -> Vec<ReindeerStats> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let fly_speed = parts[3].parse::<u64>().unwrap();
            let fly_length = parts[6].parse::<u64>().unwrap();
            let rest_length = parts[parts.len() - 2].parse::<u64>().unwrap();
            ReindeerStats {
                fly_speed,
                fly_length,
                rest_length,
            }
        })
        .collect()
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let stats = parse_input(input);
        let max_traveled = stats.iter().map(|s| distance_after(s, 2503)).max().unwrap();
        format!("{}", max_traveled)
    }

    fn star2(&self, input: &str) -> String {
        let stats = parse_input(input);
        let mut points = vec![0; stats.len()];
        for secs in 1..=2503 {
            let (winner_idx, _) = stats
                .iter()
                .enumerate()
                .map(|(i, s)| (i, distance_after(s, secs)))
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            points[winner_idx] += 1;
        }
        format!("{}", points.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        let stats = parse_input(input);
        let max_traveled = stats.iter().map(|s| distance_after(s, 1000)).max().unwrap();
        assert_eq!(max_traveled, 1120);
    }
}
