use std::io::{BufRead, BufReader};

use aoc2022::days::day2::{calculate_match_score, solve, calculate_match_score2, solve2};
use stringreader::StringReader;

const TEST_INPUT: &str = "A Y
B X
C Z";

#[test]
fn correct_match_score() {
    let matches: Vec<&str> = TEST_INPUT.split_terminator("\n").collect();
    assert_eq!(8, calculate_match_score(matches[0]));
    assert_eq!(1, calculate_match_score(matches[1]));
    assert_eq!(6, calculate_match_score(matches[2]));
}

#[test]
fn correct_match_score2() {
    let matches: Vec<&str> = TEST_INPUT.split_terminator("\n").collect();
    assert_eq!(4, calculate_match_score2(matches[0]));
    assert_eq!(1, calculate_match_score2(matches[1]));
    assert_eq!(7, calculate_match_score2(matches[2]));
}

#[test]
fn correct_solution() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(15, solve(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}

#[test]
fn correct_solution2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(12, solve2(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}