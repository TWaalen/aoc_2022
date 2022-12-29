use aoc2022::days::day6::find_first_start_of_packet_marker_index;

const TEST_INPUT: [&str; 5] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
];

#[test]
fn correct_start_of_packet_marker_index() {
    assert_eq!(7, find_first_start_of_packet_marker_index(TEST_INPUT[0]));
    assert_eq!(5, find_first_start_of_packet_marker_index(TEST_INPUT[1]));
    assert_eq!(6, find_first_start_of_packet_marker_index(TEST_INPUT[2]));
    assert_eq!(10, find_first_start_of_packet_marker_index(TEST_INPUT[3]));
    assert_eq!(11, find_first_start_of_packet_marker_index(TEST_INPUT[4]));
}