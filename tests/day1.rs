use std::io::{BufReader, BufRead};

use aoc2022::days::day1::{calculate_calories_for_elf, solve, solve2};
use stringreader::StringReader;

const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn correct_inventory_sum() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(6000, calculate_calories_for_elf(&mut input));
    assert_eq!(4000, calculate_calories_for_elf(&mut input));
    assert_eq!(11000, calculate_calories_for_elf(&mut input));
    assert_eq!(24000, calculate_calories_for_elf(&mut input));
    assert_eq!(10000, calculate_calories_for_elf(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}

#[test]
fn correct_solution() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(24000, solve(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}

#[test]
fn correct_solution2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(45000, solve2(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}