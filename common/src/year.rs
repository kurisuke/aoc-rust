use crate::day::Day;

pub trait Year {
    fn get_day(&self, day_no: usize) -> Option<Box<dyn Day>>;
    fn year_no(&self) -> usize;
}
