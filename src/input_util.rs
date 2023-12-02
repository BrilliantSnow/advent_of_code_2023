use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(directory: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(directory).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}
