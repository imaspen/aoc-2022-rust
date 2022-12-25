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
        11 => day = Box::new(days::day_11::Day11::new()),
        12 => day = Box::new(days::day_12::Day12::new()),
        13 => day = Box::new(days::day_13::Day13::new()),
        14 => day = Box::new(days::day_14::Day14::new()),
        15 => day = Box::new(days::day_15::Day15::new()),
        16 => day = Box::new(days::day_16::Day16::new()),
        17 => day = Box::new(days::day_17::Day17::new()),
        18 => day = Box::new(days::day_18::Day18::new()),
        19 => day = Box::new(days::day_19::Day19::new()),
        20 => day = Box::new(days::day_20::Day20::new()),
        21 => day = Box::new(days::day_21::Day21::new()),
        22 => day = Box::new(days::day_22::Day22::new()),
        23 => day = Box::new(days::day_23::Day23::new()),
        24 => day = Box::new(days::day_24::Day24::new()),
        25 => day = Box::new(days::day_25::Day25::new()),
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
