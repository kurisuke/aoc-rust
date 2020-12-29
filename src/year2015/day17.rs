use crate::day::Day;
use crate::util::combos::get_combos;
use itertools::Itertools;

pub struct Day17 {}

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
