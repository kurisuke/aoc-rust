use common::day::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct Day04 {}

struct Night {
    guard: u32,
    sleep_intervals: Vec<(i32, i32)>,
}

fn parse_input(input: &str) -> Vec<Night> {
    let re_prefix = r"\[1518-(\d\d)-(\d\d) (\d\d):(\d\d)\] ";
    let re_guard = Regex::new(&format!(r"{}Guard #(\d+) begins shift", re_prefix)).unwrap();
    let re_falls_asleep = Regex::new(&format!(r"{}falls asleep", re_prefix)).unwrap();
    let re_wakes_up = Regex::new(&format!(r"{}wakes up", re_prefix)).unwrap();

    let mut nights = Vec::new();
    let lines: Vec<_> = input.lines().sorted().collect();
    let mut i = 0;
    while i < lines.len() {
        let caps = re_guard.captures(lines[i]).unwrap();
        let guard = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        i += 1;
        let mut sleep_intervals = vec![];
        while (i + 1) < lines.len() && !lines[i].contains("Guard") {
            let caps = re_falls_asleep.captures(lines[i]).unwrap();
            let asleep_minute = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let caps = re_wakes_up.captures(lines[i + 1]).unwrap();
            let wakeup_minute = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
            sleep_intervals.push((asleep_minute, wakeup_minute));
            i += 2;
        }

        nights.push(Night {
            guard,
            sleep_intervals,
        });
    }

    nights
}

fn most_asleep(guards: &HashMap<u32, Vec<u32>>) -> u32 {
    *guards
        .iter()
        .map(|(guard, minutes)| (guard, minutes.iter().sum::<u32>()))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0
}

fn guards_frequency(nights: &[Night]) -> HashMap<u32, Vec<u32>> {
    let mut guards = HashMap::new();
    for night in nights {
        let e = guards.entry(night.guard).or_insert_with(|| vec![0; 60]);
        for interval in night.sleep_intervals.iter() {
            for i in interval.0..interval.1 {
                e[i as usize] += 1;
            }
        }
    }
    guards
}

fn find_guard_star1(nights: &[Night]) -> u32 {
    let guards = guards_frequency(&nights);
    let max_guard = most_asleep(&guards);
    let max_guard_minutes = guards.get(&max_guard).unwrap();
    let max_minute = max_guard_minutes
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;

    max_guard * (max_minute as u32)
}

fn find_guard_star2(nights: &[Night]) -> u32 {
    let guards = guards_frequency(&nights);
    let guard_max_minute: Vec<_> = guards
        .iter()
        .map(|(guard, minutes)| {
            let max_minute = minutes
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            (guard, max_minute.0, max_minute.1)
        })
        .collect();
    let result_guard = guard_max_minute
        .iter()
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();
    result_guard.0 * (result_guard.1 as u32)
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let nights = parse_input(input);
        format!("{}", find_guard_star1(&nights))
    }

    fn star2(&self, input: &str) -> String {
        let nights = parse_input(input);
        format!("{}", find_guard_star2(&nights))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day04 {};
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;
        assert_eq!(d.star1(input), "240");
        assert_eq!(d.star2(input), "4455");
    }
}
