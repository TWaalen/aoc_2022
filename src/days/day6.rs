use std::io::{Read, BufReader};

pub fn find_first_start_of_packet_marker_index(data: &str) -> i32 {
    for i in 0..data.len() {
        let check_slice = &data[i..i + 4];
        let mut found_start_of_packet_marker = true;
        for j in 0..4 {
            let last_index_of_char = check_slice.rfind(check_slice.chars().nth(j).unwrap()).unwrap();
            if last_index_of_char != j {
                found_start_of_packet_marker = false;
                break;
            }
        }

        if found_start_of_packet_marker {
            return i as i32 + 4;
        }
    }

    return -1;
}

pub fn solve<R>(input: &mut BufReader<R>) -> String where R : Read {
    let mut data = Default::default();
    input.read_to_string(&mut data).expect("Failed to read input");

    return find_first_start_of_packet_marker_index(&data).to_string();
}