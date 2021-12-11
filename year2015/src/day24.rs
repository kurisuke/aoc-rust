use common::day::Day;
use util::combos::get_combos;

pub struct Day24 {}

fn min_qe(packages: &[usize], divisor: usize) -> usize {
    let target = packages.iter().sum::<usize>() / divisor;
    let combos = get_combos(target, packages);
    let min_len = combos.iter().map(|combo| combo.len()).min().unwrap();
    let combos_min_len: Vec<_> = combos
        .iter()
        .filter(|combo| combo.len() == min_len)
        .collect();
    let min_qe = combos_min_len
        .iter()
        .map(|combo| combo.iter().product::<usize>())
        .min()
        .unwrap();
    min_qe
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let packages: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
        format!("{}", min_qe(&packages, 3))
    }

    fn star2(&self, input: &str) -> String {
        let packages: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
        format!("{}", min_qe(&packages, 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(min_qe(&packages, 3), 99);
        assert_eq!(min_qe(&packages, 4), 44);
    }
}
