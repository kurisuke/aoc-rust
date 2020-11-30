mod day;
mod day01;

use day::Day;

fn get_day(day_no: usize) -> Option<Box<dyn Day>> {
    match day_no {
        1 => Some(Box::new(day01::Day01 {})),
        _ => None,
    }
}

fn run_day(day_no: usize) {
    if let Some(day) = get_day(day_no) {
        println!("Day:      {:02}", day_no);
        let input = load_input(day_no, "");
        let res1 = day.star1(&input);
        let res2 = day.star2(&input);
        println!("Result 1: {}", res1);
        println!("Result 2: {}", res2);
    } else {
        println!("Day {} not implemented!", day_no);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day_no = args[1].parse::<usize>().unwrap();

    run_day(day_no);
}

fn load_input(day_no: usize, suffix: &str) -> String {
    let filename = format!("input/day{:02}{}", day_no, suffix);
    std::fs::read_to_string(filename).unwrap()
}
