use common::day::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Steps = HashMap<char, Vec<char>>;

pub struct Day07 {}

#[derive(Clone)]
struct Worker {
    task: char,
    end_time: usize,
}

fn parse_input(input: &str) -> Steps {
    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    let mut steps = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let prev = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let step = caps.get(2).unwrap().as_str().chars().next().unwrap();
        let e = steps.entry(step).or_insert_with(|| vec![]);
        e.push(prev);
    }

    // find the start step
    let next_steps: HashSet<_> = steps.keys().cloned().collect();
    let prev_steps: HashSet<_> = steps
        .values()
        .map(|v| v.iter().cloned())
        .flatten()
        .collect();
    let no_prev: HashSet<_> = prev_steps.difference(&next_steps).collect();
    for step in no_prev {
        steps.insert(*step, vec![]);
    }

    steps
}

fn sort_steps(steps: &Steps) -> String {
    let mut steps = steps.clone();
    let mut order = String::new();
    while !steps.is_empty() {
        let next = *steps
            .iter()
            .filter(|(_, prev)| prev.is_empty())
            .map(|(k, _)| k)
            .min()
            .unwrap();
        steps.remove(&next);

        for prev in steps.values_mut() {
            if let Some(pos) = prev.iter().position(|x| x == &next) {
                prev.remove(pos);
            }
        }

        order.push(next);
    }
    order
}

fn work_time(steps: &Steps, num_workers: usize, base: usize) -> usize {
    let order = sort_steps(&steps);
    let mut todo = steps.clone();
    let mut done = HashSet::new();

    let mut workers: Vec<Option<Worker>> = vec![None; num_workers];
    let mut time = 0;

    while !todo.is_empty() {
        // handle finished tasks
        for finished_worker in workers
            .iter_mut()
            .filter(|w| w.is_some() && w.as_ref().unwrap().end_time == time)
        {
            done.insert(finished_worker.as_ref().unwrap().task);
            *finished_worker = None;
        }

        // try to distribute tasks to free workers
        while workers.iter().any(|x| x.is_none()) {
            if let Some(next_task) = order.chars().find(|t| {
                todo.contains_key(&t) && todo.get(&t).unwrap().iter().all(|p| done.contains(p))
            }) {
                // insert to a free worker
                let free_worker_pos = workers.iter().position(|w| w.is_none()).unwrap();
                let end_time = time + 1 + base + (next_task as u8 - b'A') as usize;
                workers[free_worker_pos] = Some(Worker {
                    task: next_task,
                    end_time,
                });
                todo.remove(&next_task);
            } else {
                // break if we can do no more tasks although there might be free workers
                break;
            }
        }

        // sleep until a worker gets idle
        time = workers
            .iter()
            .filter_map(|x| x.as_ref())
            .map(|x| x.end_time)
            .min()
            .unwrap();
    }
    // return the highest end time of all workers (time when all work is done)
    workers
        .iter()
        .filter_map(|x| x.as_ref())
        .map(|x| x.end_time)
        .max()
        .unwrap()
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let steps = parse_input(input);
        sort_steps(&steps)
    }

    fn star2(&self, input: &str) -> String {
        let steps = parse_input(input);
        format!("{}", work_time(&steps, 5, 60))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day07 {};
        let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;
        assert_eq!(d.star1(input), "CABDFE");

        let steps = parse_input(input);
        assert_eq!(work_time(&steps, 2, 0), 15);
    }
}
