use std::io::{BufReader, BufRead};

use aoc2022::days::day3::{get_item_priority, find_misplaced_item_in_rucksack, find_badge_item_in_rucksacks, calculate_misplaced_priority_of_rucksacks, calculate_badge_priority_of_rucksacks};
use stringreader::StringReader;

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn correct_item_priority() {
    assert_eq!(27, get_item_priority('A'));
    assert_eq!(52, get_item_priority('Z'));
    assert_eq!(1, get_item_priority('a'));
    assert_eq!(26, get_item_priority('z'));
}

#[test]
fn correct_misplaced_item() {
    let mut rucksacks = TEST_INPUT.lines();
    assert_eq!('p', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
    assert_eq!('L', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
    assert_eq!('P', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
    assert_eq!('v', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
    assert_eq!('t', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
    assert_eq!('s', find_misplaced_item_in_rucksack(rucksacks.next().unwrap()));
}

#[test]
fn correct_badge_item() {
    let mut lines = TEST_INPUT.lines().map(|line| line.to_string());
    let mut rucksacks = lines.by_ref().take(3).collect::<Vec<_>>();
    assert_eq!('r', find_badge_item_in_rucksacks(&rucksacks));
    rucksacks = lines.take(3).collect::<Vec<_>>();
    assert_eq!('Z', find_badge_item_in_rucksacks(&rucksacks));
}

#[test]
fn correct_solution() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(157, calculate_misplaced_priority_of_rucksacks(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}

#[test]
fn correct_solution2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(70, calculate_badge_priority_of_rucksacks(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}