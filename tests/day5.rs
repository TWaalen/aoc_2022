use std::io::{BufReader, BufRead, Read};

use aoc2022::days::day5::{parse_crate_stacks, parse_move_instructions, MoveInstruction, execute_move_instructions, execute_move_instructions_retain_order, solve, solve2};
use stringreader::StringReader;

const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn correct_parsed_stacks() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    let crate_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    assert_eq!(crate_stacks, parse_crate_stacks(&mut input));
}

#[test]
fn correct_parsed_move_instructions() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    input.by_ref().lines().take_while(|line| !line.as_ref().unwrap().is_empty()).for_each(drop);
    let move_instructions = vec![
        MoveInstruction { moves: 1, from: 1, to: 0 },
        MoveInstruction { moves: 3, from: 0, to: 2 },
        MoveInstruction { moves: 2, from: 1, to: 0 },
        MoveInstruction { moves: 1, from: 0, to: 1 }
    ];
    assert_eq!(move_instructions, parse_move_instructions(&mut input))
}

#[test]
fn correct_execute_move_instructions() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    let mut crate_stacks = parse_crate_stacks(&mut input);
    let move_instructions = parse_move_instructions(&mut input);
    execute_move_instructions(&mut crate_stacks, &move_instructions);
    assert_eq!(Some(&'C'), crate_stacks[0].last());
    assert_eq!(Some(&'M'), crate_stacks[1].last());
    assert_eq!(Some(&'Z'), crate_stacks[2].last());
}

#[test]
fn correct_execute_move_instructions2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    let mut crate_stacks = parse_crate_stacks(&mut input);
    let move_instructions = parse_move_instructions(&mut input);
    execute_move_instructions_retain_order(&mut crate_stacks, &move_instructions);
    assert_eq!(Some(&'M'), crate_stacks[0].last());
    assert_eq!(Some(&'C'), crate_stacks[1].last());
    assert_eq!(Some(&'D'), crate_stacks[2].last());
}

#[test]
fn correct_solution() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!("CMZ", solve(&mut input));
}

#[test]
fn correct_solution2() {
    let mut input = BufReader::new(StringReader::new(TEST_INPUT));
    assert_eq!("MCD", solve2(&mut input));
}
