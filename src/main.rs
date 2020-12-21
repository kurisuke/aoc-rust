mod day;
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
mod day21;
mod grid2d;

use day::Day;

use std::time::Instant;

fn get_day(day_no: usize) -> Option<Box<dyn Day>> {
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
        21 => Some(Box::new(day21::Day21 {})),
        _ => None,
    }
}

fn get_day_no_max() -> usize {
    let mut day_no = 0;
    while get_day(day_no + 1).is_some() {
        day_no += 1;
    }
    day_no
}

fn run_day(day_no: usize) {
    if let Some(day) = get_day(day_no) {
        println!("Day: {:02}", day_no);
        let input = load_input(day_no);

        for star_no in 1..3 {
            let start = Instant::now();
            let res = match star_no {
                1 => day.star1(&input),
                2 => day.star2(&input),
                _ => format!("invalid star_no: {}", star_no),
            };
            println!(
                "star {}  time: {:>10} µs    res: {}",
                star_no,
                start.elapsed().as_micros(),
                res
            );
        }
    } else {
        println!("Day {} not implemented!", day_no);
    }
}

fn run_all() {
    let start_all = Instant::now();
    for day_no in 0..get_day_no_max() {
        run_day(day_no + 1);
    }
    println!("\ntotal   time: {:>10} µs", start_all.elapsed().as_micros());
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 1 {
        run_day(get_day_no_max());
    } else {
        match args[1].parse::<usize>() {
            Ok(d) => run_day(d),
            Err(_) => match args[1].as_ref() {
                "all" => {
                    run_all();
                }
                _ => {
                    println!("Invalid parameter: {}", args[1]);
                }
            },
        }
    }
}

fn load_input(day_no: usize) -> String {
    let filename = format!("input/day{:02}.input", day_no);
    std::fs::read_to_string(filename).unwrap()
}
