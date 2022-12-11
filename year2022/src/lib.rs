use common::day::Day;
use common::year::Year;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

pub struct Year2022 {}

impl Year for Year2022 {
    fn get_day(&self, day_no: usize) -> Option<Box<dyn Day>> {
        match day_no {
            1 => Some(Box::new(day01::Day01 {})),
            2 => Some(Box::new(day02::Day02 {})),
            3 => Some(Box::new(day03::Day03 {})),
            4 => Some(Box::new(day04::Day04 {})),
            5 => Some(Box::new(day05::Day05 {})),
            6 => Some(Box::new(day06::Day06 {})),
            7 => Some(Box::new(day07::Day07 {})),
            8 => Some(Box::new(day08::Day08 {})),
            9 => Some(Box::new(day09::Day09 {})),
            10 => Some(Box::new(day10::Day10 {})),
            11 => Some(Box::new(day11::Day11 {})),
            _ => None,
        }
    }

    fn year_no(&self) -> usize {
        2022
    }
}
