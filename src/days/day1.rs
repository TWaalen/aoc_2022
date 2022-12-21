use std::io::{BufReader, Read, BufRead};

pub fn calculate_calories_for_elf<R>(input: &mut BufReader<R>) -> i32 where R: Read {
    let mut calories = 0;
    for line in input.lines() {
        let line = line.expect("Unable to get line from input");
        if line.is_empty() {
            return calories;
        }

        if let Ok(result) = line.parse::<i32>() {
            calories += result;
        }
    }

    return calories;
}

pub fn solve<R>(input: &mut BufReader<R>) -> i32 where R: Read {
    let mut maximum_calories = 0;
    
    while !input.fill_buf().unwrap().is_empty() {
        let elf_calories = calculate_calories_for_elf(input);
        if elf_calories > maximum_calories {
            maximum_calories = elf_calories;
        }
    }
    
    
    return maximum_calories;
}

pub fn solve2<R>(input: &mut BufReader<R>) -> i32 where R: Read {
    let mut result = Vec::new();
    
    while !input.fill_buf().unwrap().is_empty() {
        result.push(calculate_calories_for_elf(input));

        if result.len() > 3 {
            result.sort_unstable_by(|a, b| b.cmp(a));
            result.pop();
        }
    }
    
    return result.iter().sum::<i32>();
}