use std::io::{Read, BufReader, BufRead};

pub fn get_item_priority(character: char) -> i32 {
    if character.is_ascii_lowercase() {
        return character as i32 - 96;
    }

    return character as i32 - 38;
}

pub fn find_badge_item_in_rucksacks(rucksacks: &Vec<String>) -> char {
    for character in rucksacks[0].chars() {
        if rucksacks[1].contains(character) && rucksacks[2].contains(character) {
            return character;
        }
    }

    panic!("No badge!");
}

pub fn find_misplaced_item_in_rucksack(rucksack: &str) -> char {
    let compartments = rucksack.split_at(rucksack.len() / 2);
    for character in compartments.0.chars() {
        if compartments.1.contains(character) {
            return character;
        }
    }

    panic!("No misplaced item!");
}

pub fn calculate_misplaced_priority_of_rucksacks<R>(reader: &mut BufReader<R>) -> i32 where R: Read {
    let mut total_priority: i32 = 0;
    for line in reader.lines() {
        let rucksack = line.expect("No line");

        let duplicate_item = find_misplaced_item_in_rucksack(&rucksack);
        total_priority += get_item_priority(duplicate_item);
    }

    return total_priority;
}

pub fn calculate_badge_priority_of_rucksacks<R>(reader: &mut BufReader<R>) -> i32 where R: Read {
    let mut priority: i32 = 0;
    while !reader.fill_buf().unwrap().is_empty() {
        let mut lines_iter = reader.lines();
        let lines = lines_iter.by_ref().take(3).map(|line| line.unwrap()).collect::<Vec<_>>();
        let badge_item = find_badge_item_in_rucksacks(&lines);
        priority += get_item_priority(badge_item);
    }

    return priority;
}
