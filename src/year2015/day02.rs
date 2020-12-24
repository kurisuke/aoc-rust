use crate::day::Day;

pub struct Day02 {}

type Dims = (usize, usize, usize);

fn parse_input(input: &str) -> Vec<Dims> {
    input
        .lines()
        .map(|l| {
            let v: Vec<_> = l.split('x').map(|x| x.parse::<usize>().unwrap()).collect();
            (v[0], v[1], v[2])
        })
        .collect()
}

fn wrapping_paper(package: Dims) -> usize {
    let side_areas = vec![
        package.0 * package.1,
        package.0 * package.2,
        package.1 * package.2,
    ];
    let smallest_side = side_areas.iter().min().unwrap();
    side_areas.iter().map(|a| 2 * a).sum::<usize>() + smallest_side
}

fn ribbon(package: Dims) -> usize {
    let mut sides = vec![package.0, package.1, package.2];
    sides.sort_unstable();
    2 * (sides[0] + sides[1]) + sides.iter().product::<usize>()
}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let packages = parse_input(input);
        let total: usize = packages.into_iter().map(wrapping_paper).sum();
        format!("{}", total)
    }

    fn star2(&self, input: &str) -> String {
        let packages = parse_input(input);
        let total: usize = packages.into_iter().map(ribbon).sum();
        format!("{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let d = Day02 {};
        assert_eq!(d.star1("2x3x4"), "58");
        assert_eq!(d.star1("1x1x10"), "43");
    }

    #[test]
    fn part2() {
        let d = Day02 {};
        assert_eq!(d.star2("2x3x4"), "34");
        assert_eq!(d.star2("1x1x10"), "14");
    }
}
