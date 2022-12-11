use crate::days::Day;
use std::process::ExitCode;

mod days;
mod utils;

fn print_malformed_args() {
    println!("Args are malformed, expected: aoc (day) (part)");
}

fn main() -> Result<(), ExitCode> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print_malformed_args();
        return Err(ExitCode::FAILURE);
    }
    let day_num = args[1].parse::<u8>().unwrap_or(0);
    let part_num = args[2].parse::<u8>().unwrap_or(0);

    let mut day: Box<dyn Day>;
    match day_num {
        1 => day = Box::new(days::day_01::Day01::new()),
        2 => day = Box::new(days::day_02::Day02::new()),
        3 => day = Box::new(days::day_03::Day03::new()),
        4 => day = Box::new(days::day_04::Day04::new()),
        5 => day = Box::new(days::day_05::Day05::new()),
        6 => day = Box::new(days::day_06::Day06::new()),
        7 => day = Box::new(days::day_07::Day07::new()),
        8 => day = Box::new(days::day_08::Day08::new()),
        9 => day = Box::new(days::day_09::Day09::new()),
        10 => day = Box::new(days::day_10::Day10::new()),
        11..=25 => todo!(),
        _ => {
            print_malformed_args();
            return Err(ExitCode::FAILURE);
        }
    }

    match part_num {
        1 => println!("{}", day.part_1()),
        2 => println!("{}", day.part_2()),
        _ => {
            print_malformed_args();
            return Err(ExitCode::FAILURE);
        }
    }

    return Ok(());
}
