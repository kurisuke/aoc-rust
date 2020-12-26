use crate::day::Day;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Day17 {}

fn get_combos(target: usize, containers: &[usize]) -> Vec<Vec<usize>> {
    let mut combos = vec![];
    for (idx, container) in containers.iter().enumerate() {
        match target.cmp(container) {
            Ordering::Equal => {
                combos.push(vec![*container]);
            }
            Ordering::Greater => {
                let new_target = target - container;
                let new_containers: Vec<usize> = containers[idx + 1..].iter().copied().collect();
                for sub_combo in get_combos(new_target, &new_containers) {
                    let mut combo = vec![*container];
                    combo.extend(&sub_combo);
                    combos.push(combo);
                }
            }
            Ordering::Less => {}
        }
    }
    combos
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let containers: Vec<_> = input
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .sorted()
            .rev()
            .collect();
        format!("{}", get_combos(150, &containers).len())
    }

    fn star2(&self, input: &str) -> String {
        let containers: Vec<_> = input
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .sorted()
            .rev()
            .collect();
        let combos = get_combos(150, &containers);
        let min_len = combos.iter().map(|combo| combo.len()).min().unwrap();
        let min_combos = combos.iter().filter(|combo| combo.len() == min_len).count();
        format!("{}", min_combos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let containers = [20, 15, 10, 5, 5];
        assert_eq!(get_combos(25, &containers).len(), 4);

        let combos = get_combos(25, &containers);
        let min_len = combos.iter().map(|combo| combo.len()).min().unwrap();
        let min_combos = combos.iter().filter(|combo| combo.len() == min_len).count();
        assert_eq!(min_combos, 3);
    }
}
