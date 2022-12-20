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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

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
            12 => Some(Box::new(day12::Day12 {})),
            13 => Some(Box::new(day13::Day13 {})),
            14 => Some(Box::new(day14::Day14 {})),
            15 => Some(Box::new(day15::Day15 {})),
            16 => Some(Box::new(day16::Day16 {})),
            17 => Some(Box::new(day17::Day17 {})),
            18 => Some(Box::new(day18::Day18 {})),
            19 => Some(Box::new(day19::Day19 {})),
            20 => Some(Box::new(day20::Day20 {})),
            _ => None,
        }
    }

    fn year_no(&self) -> usize {
        2022
    }
}
