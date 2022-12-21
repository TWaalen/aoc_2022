use std::io::{Read, BufReader, BufRead};

pub fn get_item_priority(character: char) -> i32 {
    if character.is_ascii_lowercase() {
        return character as i32 - 96;
    }

    return character as i32 - 38;
}

pub fn find_badge_item_in_rucksacks(rucksacks: &[String; 3]) -> char {
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
    for lines in reader.lines().array_chunks::<3>() {
        let rucksacks = lines.map(|line| line.expect("No line"));
        let badge_item = find_badge_item_in_rucksacks(&rucksacks);
        priority += get_item_priority(badge_item);
    }

    return priority;
}
