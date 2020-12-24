use crate::day::Day;
use crate::year::Year;

mod day01;

pub struct Year2015 {}

impl Year for Year2015 {
    fn get_day(&self, day_no: usize) -> Option<Box<dyn Day>> {
        match day_no {
            1 => Some(Box::new(day01::Day01 {})),
            _ => None,
        }
    }

    fn year_no(&self) -> usize {
        2015
    }
}
