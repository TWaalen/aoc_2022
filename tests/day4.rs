use std::io::{BufReader, BufRead};

use aoc2022::days::day4::{get_section_bounds, SectionBounds, has_fully_contained_section, count_fully_contained_sections, do_sections_overlap, count_overlapping_sections};
use stringreader::StringReader;

const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn correct_section_bounds() {
    let mut section_bound_pairs = TEST_INPUT.lines();
    assert_eq!(vec!(SectionBounds { left: 2, right: 4 }, SectionBounds { left: 6, right: 8 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
    assert_eq!(vec!(SectionBounds { left: 2, right: 3 }, SectionBounds { left: 4, right: 5 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
    assert_eq!(vec!(SectionBounds { left: 5, right: 7 }, SectionBounds { left: 7, right: 9 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
    assert_eq!(vec!(SectionBounds { left: 2, right: 8 }, SectionBounds { left: 3, right: 7 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
    assert_eq!(vec!(SectionBounds { left: 6, right: 6 }, SectionBounds { left: 4, right: 6 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
    assert_eq!(vec!(SectionBounds { left: 2, right: 6 }, SectionBounds { left: 4, right: 8 }).as_slice(), get_section_bounds(section_bound_pairs.next().unwrap()).as_slice());
}

#[test]
fn correct_fully_contained_check() {
    let mut section_bound_pairs = TEST_INPUT.lines();
    assert_eq!(false, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
    assert_eq!(false, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
    assert_eq!(false, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
    assert_eq!(true, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
    assert_eq!(true, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
    assert_eq!(false, has_fully_contained_section(get_section_bounds(section_bound_pairs.next().unwrap())));
}

#[test]
fn correct_overlap_check() {
    let mut section_bound_pairs = TEST_INPUT.lines();
    let mut bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(false, do_sections_overlap(&bounds[0], &bounds[1]));
    bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(false, do_sections_overlap(&bounds[0], &bounds[1]));
    bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(true, do_sections_overlap(&bounds[0], &bounds[1]));
    bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(true, do_sections_overlap(&bounds[0], &bounds[1]));
    bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(true, do_sections_overlap(&bounds[0], &bounds[1]));
    bounds = get_section_bounds(section_bound_pairs.next().unwrap());
    assert_eq!(true, do_sections_overlap(&bounds[0], &bounds[1]));
}

#[test]
fn correct_solution() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(2, count_fully_contained_sections(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}

#[test]
fn correct_solution2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!(4, count_overlapping_sections(&mut input));
    assert!(input.fill_buf().unwrap().is_empty())
}