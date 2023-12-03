use common::day::Day;
use common::year::Year;

mod day01;
mod day02;
mod day03;

pub struct Year2023 {}

impl Year for Year2023 {
    fn get_day(&self, day_no: usize) -> Option<Box<dyn Day>> {
        match day_no {
            1 => Some(Box::new(day01::Day01 {})),
            2 => Some(Box::new(day02::Day02 {})),
            3 => Some(Box::new(day03::Day03 {})),
            _ => None,
        }
    }

    fn year_no(&self) -> usize {
        2023
    }
}
