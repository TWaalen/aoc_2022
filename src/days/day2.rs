use std::io::{BufReader, Read, BufRead};

pub fn calculate_match_score(line: &str) -> i32 {
    let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
    let opponent_score = split[0].bytes().nth(0).unwrap() as i32 - 64;
    
    let my_score = split[1].bytes().nth(0).unwrap() as i32 - 87;

    if my_score == opponent_score {
        return my_score + 3;
    } else if (my_score == 1 && opponent_score == 3) || (my_score == 2 && opponent_score == 1) || (my_score == 3 && opponent_score == 2)  {
        return my_score + 6;
    } else {
        return my_score;
    }
}

pub fn calculate_match_score2(line: &str) -> i32 {
    let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
    let opponent_score = split[0].bytes().nth(0).unwrap() as i32 - 64;

    let instruction = split[1];
    if instruction == "X" {
        if opponent_score == 1 {
            return 3
        } else {
            return opponent_score - 1;
        }
    } else if instruction == "Y" {
        return opponent_score + 3;
    } else if instruction == "Z" {
        if opponent_score == 3 {
            return 7;
        } else {
            return opponent_score + 7;
        }
    }

    return 0;
}

pub fn solve<R>(input: &mut BufReader<R>) -> i32 where R: Read {
    let mut total_score = 0;
    for line in input.lines() {
        let line = line.expect("No line");
        total_score += calculate_match_score(&line);
    }

    return total_score;
}

pub fn solve2<R>(input: &mut BufReader<R>) -> i32 where R: Read {
    let mut total_score = 0;
    for line in input.lines() {
        let line = line.expect("No line");
        total_score += calculate_match_score2(&line);
    }

    return total_score;
}
