use common::day::Day;

pub struct Day01 {}

fn fuel_pt1(mass: usize) -> usize {
    mass / 3 - 2
}

fn fuel_pt2(mass: usize) -> usize {
    if mass > 8 {
        let add_fuel = mass / 3 - 2;
        add_fuel + fuel_pt2(add_fuel)
    } else {
        0
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let masses = parse_input(input);
        let sum_fuels = masses.into_iter().map(fuel_pt1).sum::<usize>();
        format!("{}", sum_fuels)
    }

    fn star2(&self, input: &str) -> String {
        let masses = parse_input(input);
        let sum_fuels = masses.into_iter().map(fuel_pt2).sum::<usize>();
        format!("{}", sum_fuels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        assert_eq!(fuel_pt1(12), 2);
        assert_eq!(fuel_pt1(14), 2);
        assert_eq!(fuel_pt1(1969), 654);
        assert_eq!(fuel_pt1(100756), 33583);
    }

    #[test]
    fn star2() {
        assert_eq!(fuel_pt2(14), 2);
        assert_eq!(fuel_pt2(1969), 966);
        assert_eq!(fuel_pt2(100756), 50346);
    }
}
