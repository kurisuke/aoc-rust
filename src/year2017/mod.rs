use crate::day::Day;
use crate::year::Year;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub struct Year2017 {}

impl Year for Year2017 {
    fn get_day(&self, day_no: usize) -> Option<Box<dyn Day>> {
        match day_no {
            1 => Some(Box::new(day01::Day01 {})),
            2 => Some(Box::new(day02::Day02 {})),
            3 => Some(Box::new(day03::Day03 {})),
            4 => Some(Box::new(day04::Day04 {})),
            5 => Some(Box::new(day05::Day05 {})),
            6 => Some(Box::new(day06::Day06 {})),
            _ => None,
        }
    }

    fn year_no(&self) -> usize {
        2017
    }
}
