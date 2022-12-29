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

fn get_solver_fn<R>(day: i32, part: i32) -> fn(&mut BufReader<R>) -> i32 where R: Read {
    return match (day, part) {
        (1, 0) => days::day1::solve,
        (1, 1) => days::day1::solve2,
        (2, 0) => days::day2::solve,
        (2, 1) => days::day2::solve2,
        (3, 0) => days::day3::calculate_misplaced_priority_of_rucksacks,
        (3, 1) => days::day3::calculate_badge_priority_of_rucksacks,
        (4, 0) => days::day4::count_fully_contained_sections,
        (4, 1) => days::day4::count_overlapping_sections,
        (_, _) => panic!("No solver found for day {} part {}", day , part)
    }
}
