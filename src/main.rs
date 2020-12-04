mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod grid2d;

use day::Day;

use std::time::Instant;

fn get_day(day_no: usize) -> Option<Box<dyn Day>> {
    match day_no {
        1 => Some(Box::new(day01::Day01 {})),
        2 => Some(Box::new(day02::Day02 {})),
        3 => Some(Box::new(day03::Day03 {})),
        4 => Some(Box::new(day04::Day04 {})),
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
            println!("Result {} [{:?}]:\n{}", star_no, start.elapsed(), res);
        }
    } else {
        println!("Day {} not implemented!", day_no);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day_no = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        get_day_no_max()
    };

    run_day(day_no);
}

fn load_input(day_no: usize) -> String {
    let filename = format!("input/day{:02}.input", day_no);
    std::fs::read_to_string(filename).unwrap()
}
