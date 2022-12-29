use std::{io::{Read, BufReader}, usize::MAX};

pub fn find_end_of_marker_index(data: &str, marker_size: usize) -> usize {
    for i in 0..data.len() {
        let check_slice = &data[i..i + marker_size];
        let mut found_start_of_packet_marker = true;
        for j in 0..marker_size {
            let last_index_of_char = check_slice.rfind(check_slice.chars().nth(j).unwrap()).unwrap();
            if last_index_of_char != j {
                found_start_of_packet_marker = false;
                break;
            }
        }

        if found_start_of_packet_marker {
            return i + marker_size;
        }
    }

    return MAX;
}

pub fn solve<R>(input: &mut BufReader<R>) -> String where R : Read {
    let mut data = Default::default();
    input.read_to_string(&mut data).expect("Failed to read input");

    return find_end_of_marker_index(&data, 4).to_string();
}

pub fn solve2<R>(input: &mut BufReader<R>) -> String where R : Read {
    let mut data = Default::default();
    input.read_to_string(&mut data).expect("Failed to read input");

    return find_end_of_marker_index(&data, 14).to_string();
}