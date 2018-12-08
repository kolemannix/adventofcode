use std::fs::read_to_string;

pub fn load_file(filename: &str) -> Vec<String> {
    let contents = read_to_string(filename).expect("something went wrong reading the file");
    let lines = contents.lines();
    // We need to_owned here because we need to actually put some data on the heap!
    // Originally I was returning a Vec<&str> but my values
    // did not have a long enough lifetime. Really cool!
    lines.map(|s| s.to_owned()).collect::<Vec<String>>()
}