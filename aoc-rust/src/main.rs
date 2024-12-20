mod dl_input;

use common::year::Year;
use std::io::ErrorKind;
use std::time::Instant;

use dotenv::dotenv;

const YEAR_MAX: usize = 2024;

fn parse_year(args: &[String]) -> Option<Box<dyn Year>> {
    match args.len() {
        1..=2 => get_year(YEAR_MAX),
        3 => {
            let year_no = args[1]
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Invalid year argument: {}", args[1]));
            get_year(year_no)
        }
        _ => panic!("Invalid number of arguments!"),
    }
}

fn get_year(year_no: usize) -> Option<Box<dyn Year>> {
    match year_no {
        2015 => Some(Box::new(year2015::Year2015 {})),
        2016 => Some(Box::new(year2016::Year2016 {})),
        2017 => Some(Box::new(year2017::Year2017 {})),
        2018 => Some(Box::new(year2018::Year2018 {})),
        2019 => Some(Box::new(year2019::Year2019 {})),
        2020 => Some(Box::new(year2020::Year2020 {})),
        2021 => Some(Box::new(year2021::Year2021 {})),
        2022 => Some(Box::new(year2022::Year2022 {})),
        2023 => Some(Box::new(year2023::Year2023 {})),
        2024 => Some(Box::new(year2024::Year2024 {})),
        _ => None,
    }
}

#[allow(clippy::borrowed_box)]
fn get_day_no_max(year: &Box<dyn Year>) -> usize {
    let mut day_no = 0;
    while year.get_day(day_no + 1).is_some() {
        day_no += 1;
    }
    day_no
}

#[allow(clippy::borrowed_box)]
fn run_day(year: &Box<dyn Year>, day_no: usize) {
    if let Some(day) = year.get_day(day_no) {
        println!("Year: {:04}, Day: {:02}", year.year_no(), day_no);
        let input = load_input(year.year_no(), day_no);

        for star_no in 1..3 {
            let start = Instant::now();
            let res = match star_no {
                1 => day.star1(&input),
                2 => day.star2(&input),
                _ => format!("invalid star_no: {}", star_no),
            };
            let res = if res.find('\n').is_some() {
                format!("\n{}", res)
            } else {
                res
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

#[allow(clippy::borrowed_box)]
fn run_all(year: &Box<dyn Year>) {
    let start_all = Instant::now();
    for day_no in 0..get_day_no_max(year) {
        run_day(year, day_no + 1);
    }
    println!("\ntotal   time: {:>10} µs", start_all.elapsed().as_micros());
}

fn main() {
    dotenv().ok();

    let args: Vec<_> = std::env::args().collect();
    if let Some(year) = parse_year(&args) {
        if args.len() == 1 {
            run_day(&year, get_day_no_max(&year));
        } else {
            match args[args.len() - 1].parse::<usize>() {
                Ok(d) => run_day(&year, d),
                Err(_) => match args[args.len() - 1].as_ref() {
                    "all" => {
                        run_all(&year);
                    }
                    _ => {
                        println!("Invalid parameter: {}", args[1]);
                    }
                },
            }
        }
    } else {
        println!("Year {} not implemented!", args[1]);
    }
}

fn load_input(year_no: usize, day_no: usize) -> String {
    let filename = format!("input/year{:04}/day{:02}.input", year_no, day_no);
    match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                dl_input::download(year_no, day_no);
                std::fs::read_to_string(&filename).unwrap()
            } else {
                panic!("Error: {}", e);
            }
        }
    }
}
