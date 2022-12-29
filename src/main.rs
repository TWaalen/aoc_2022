use std::{env, process, io::{BufReader, Read}};

mod utils;
mod days;

fn main() {
    if env::args().len() < 2 {
        eprintln!("No day number provided");
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let mut part = 0;

    if args.len() > 2 {
        part = args[2].parse::<i32>().unwrap();
    }

    let mut input = utils::get_input_reader(day);
    let solver = get_solver_fn(day, part);
    println!("Solution for day {} part {}: {}", day, part, solver(&mut input));
}

fn get_solver_fn<R>(day: i32, part: i32) -> fn(&mut BufReader<R>) -> String where R: Read { 
    return match (day, part) {
        (1, 0) => |input| days::day1::solve(input).to_string(),
        (1, 1) => |input| days::day1::solve2(input).to_string(),
        (2, 0) => |input| days::day2::solve(input).to_string(),
        (2, 1) => |input| days::day2::solve2(input).to_string(),
        (3, 0) => |input| days::day3::calculate_misplaced_priority_of_rucksacks(input).to_string(),
        (3, 1) => |input| days::day3::calculate_badge_priority_of_rucksacks(input).to_string(),
        (4, 0) => |input| days::day4::count_fully_contained_sections(input).to_string(),
        (4, 1) => |input| days::day4::count_overlapping_sections(input).to_string(),
        (5, 0) => days::day5::solve,
        (5, 1) => days::day5::solve2,
        (6, 0) => days::day6::solve,
        (_, _) => panic!("No solver found for day {} part {}", day , part)
    }
}
