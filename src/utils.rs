use std::{path::Path, fs::File, io::{BufReader, self, Read}};

pub fn retrieve_input(day: i32) {
    let session_path = Path::new("session");
    if !session_path.exists() {
        panic!("Session file not found");
    }

    let mut session_file = File::open(session_path).expect("Failed to open session file");
    let mut session: String = String::new();
    session_file.read_to_string(&mut session).expect("Failed to read session file");
    let session = session.trim_end();

    let input_response = ureq::get(&format!("https://adventofcode.com/2022/day/{}/input", day))
        .set("Cookie", &format!("session={}", session))
        .call()
        .expect(&format!("Failed to retrieve input for day {}", day))
        .into_reader();
    
    let mut input_reader = BufReader::new(input_response);

    let mut output_file = File::create(&format!("input{}", day)).expect("Failed to create input file");
    io::copy(&mut input_reader, &mut output_file).expect("Failed to write input file");
}

pub fn get_input_reader(day: i32) -> BufReader<File> {
    let input_path_string = format!("input{}", day);
    let input_path = Path::new(&input_path_string);
    if !input_path.exists() {
        retrieve_input(day);
    }
    let file = File::open(input_path).expect("Failed to open file");
    return BufReader::new(file);
}
