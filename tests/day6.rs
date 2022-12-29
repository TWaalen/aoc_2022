use aoc2022::days::day6::find_end_of_marker_index;

const TEST_INPUT: [&str; 5] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
];

#[test]
fn correct_start_of_packet_marker_index() {
    assert_eq!(7, find_end_of_marker_index(TEST_INPUT[0], 4));
    assert_eq!(5, find_end_of_marker_index(TEST_INPUT[1], 4));
    assert_eq!(6, find_end_of_marker_index(TEST_INPUT[2], 4));
    assert_eq!(10, find_end_of_marker_index(TEST_INPUT[3], 4));
    assert_eq!(11, find_end_of_marker_index(TEST_INPUT[4], 4));
}
