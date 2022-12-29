use std::io::{Read, BufReader, BufRead};

#[derive(PartialEq, Debug)]
pub struct MoveInstruction {
    pub moves: i32,
    pub from: usize,
    pub to: usize
}

pub fn parse_crate_stacks<R>(input: &mut BufReader<R>) -> Vec<Vec<char>> where R: Read {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line_result in input.lines() {
        let line = line_result.unwrap();
        if line.is_empty() {
            break;
        }

        for i in (1..line.len()).step_by(4) {
            let crate_char = line.chars().nth(i).unwrap();
            if crate_char.is_ascii_digit() {
                break;
            }

            if !crate_char.is_ascii_whitespace() {
                let stack_index = (i - 1) / 4;
                if stack_index >= stacks.len() {
                    stacks.resize(stack_index + 1, Vec::new());
                }

                stacks[stack_index].push(crate_char);
            }
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());
    return stacks;
}

pub fn parse_move_instructions<R>(input: &mut BufReader<R>) -> Vec<MoveInstruction> where R: Read {
    let mut move_instructions = Vec::new();
    for line_result in input.lines() {
        let line = line_result.unwrap();
        let mut splits = line.split_ascii_whitespace();
        move_instructions.push(MoveInstruction { 
            moves: splits.nth(1).unwrap().parse::<i32>().unwrap(),
            from: splits.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            to: splits.nth(1).unwrap().parse::<usize>().unwrap() - 1
        });
    }

    return move_instructions;
}

pub fn execute_move_instructions(stacks: &mut Vec<Vec<char>>, move_instructions: &Vec<MoveInstruction>) {
    for move_instruction in move_instructions {
        for _ in 0..move_instruction.moves {
            let crate_char = stacks[move_instruction.from].pop().unwrap();
            stacks[move_instruction.to].push(crate_char);
        }
    }
}

pub fn solve<R>(input: &mut BufReader<R>) -> String where R : Read {
    let mut crate_stacks = parse_crate_stacks(input);
    let move_instructions = parse_move_instructions(input);
    execute_move_instructions(&mut crate_stacks, &move_instructions);

    return format!("{}", crate_stacks.iter().map(|stack| stack.last().unwrap()).collect::<Vec<&char>>().iter().cloned().collect::<String>());
}
