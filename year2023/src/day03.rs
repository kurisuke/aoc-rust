use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        part_numbers(grid).into_iter().sum::<u32>().to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[derive(PartialEq, Eq)]
enum NumberParseState {
    None,
    PartNumber,
    NonPartNumber,
}

fn part_numbers(grid: Grid2D<char>) -> Vec<u32> {
    let mut part_nums = vec![];

    for y in 0..grid.height() {
        let mut state = NumberParseState::None;
        let mut digit_stack = vec![];

        for x in 0..grid.width() {
            let coords = Coords { x, y };
            let val = grid.at(&coords).unwrap();

            match state {
                NumberParseState::None => {
                    if val.is_ascii_digit() {
                        digit_stack.push(val.to_digit(10).unwrap());
                        if has_symbol_neighbor(&grid, &coords) {
                            state = NumberParseState::PartNumber;
                        } else {
                            state = NumberParseState::NonPartNumber;
                        }
                    }
                }
                NumberParseState::PartNumber => {
                    if val.is_ascii_digit() {
                        digit_stack.push(val.to_digit(10).unwrap());
                    } else {
                        part_nums.push(sum_digits(&digit_stack));
                        digit_stack.clear();
                        state = NumberParseState::None;
                    }
                }
                NumberParseState::NonPartNumber => {
                    if val.is_ascii_digit() {
                        digit_stack.push(val.to_digit(10).unwrap());
                        if has_symbol_neighbor(&grid, &coords) {
                            state = NumberParseState::PartNumber;
                        }
                    } else {
                        digit_stack.clear();
                        state = NumberParseState::None;
                    }
                }
            }
        }

        // row end
        if state == NumberParseState::PartNumber {
            part_nums.push(sum_digits(&digit_stack));
        }
    }
    part_nums
}

fn has_symbol_neighbor(grid: &Grid2D<char>, coords: &Coords) -> bool {
    grid.neighbors(coords)
        .into_iter()
        .any(|x| x.is_some_and(|x| !x.is_ascii_digit() && x != &'.'))
}

fn sum_digits(digit_stack: &[u32]) -> u32 {
    let mut sum = 0;
    let mut power = 1;
    for digit in digit_stack.iter().rev() {
        sum += power * digit;
        power *= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn ex1() {
        let d = Day03 {};
        assert_eq!(d.star1(INPUT), "4361");
    }
}
