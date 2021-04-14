use common::day::Day;
use itertools::iproduct;

pub struct Day11 {}

type CoordsXy = (usize, usize);
type CellGrid = [[isize; 300]; 300];

fn power_level(cell: &CoordsXy, serno: usize) -> isize {
    let rack_id = cell.0 + 10;
    let power_level = (rack_id * cell.1 + serno) * rack_id;
    let hundred_digit = (power_level / 100) % 10;
    hundred_digit as isize - 5
}

fn calc_cell_powers(serno: usize) -> CellGrid {
    // calc power levels
    let mut cell_powers = [[0_isize; 300]; 300];
    for (x, row) in cell_powers.iter_mut().enumerate() {
        for (y, pos) in row.iter_mut().enumerate() {
            let cell = (x + 1, y + 1);
            *pos = power_level(&cell, serno);
        }
    }
    cell_powers
}

fn max_power(cell_powers: &CellGrid, square_size: usize) -> (CoordsXy, isize) {
    // get best 3x3 square
    let mut best_power = isize::MIN;
    let mut best_square = (0, 0);
    let range: Vec<usize> = (0..300).collect();
    for w_x in range.windows(square_size) {
        for w_y in range.windows(square_size) {
            let power = iproduct!(w_x, w_y).map(|(x, y)| cell_powers[*x][*y]).sum();
            if power > best_power {
                best_power = power;
                best_square = (w_x[0] + 1, w_y[0] + 1);
            }
        }
    }
    (best_square, best_power)
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let serno = input.trim().parse::<usize>().unwrap();
        let cell_powers = calc_cell_powers(serno);
        let (best_square, _) = max_power(&cell_powers, 3);
        format!("{},{}", best_square.0, best_square.1)
    }

    fn star2(&self, input: &str) -> String {
        let serno = input.trim().parse::<usize>().unwrap();
        let cell_powers = calc_cell_powers(serno);

        let mut best_power_total = isize::MIN;
        let mut best_square_total = (0, 0, 0);
        for square_size in 1..=20 {
            let (best_square, best_power) = max_power(&cell_powers, square_size);
            if best_power > best_power_total {
                best_power_total = best_power;
                best_square_total = (best_square.0, best_square.1, square_size);
            }
        }
        format!(
            "{},{},{}",
            best_square_total.0, best_square_total.1, best_square_total.2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day11 {};
        assert_eq!(d.star1("18"), "33,45");
        assert_eq!(d.star1("42"), "21,61");
    }

    #[test]
    #[ignore]
    fn star2() {
        let d = Day11 {};
        assert_eq!(d.star2("18"), "90,269,16");
        assert_eq!(d.star2("42"), "232,251,12");
    }
}
