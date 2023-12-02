use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader, Read},
};

pub fn read_file_buffered(directory: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(directory).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}

pub fn read_entre_file(directory: &str) -> String {
    return read_to_string(directory).unwrap();
}
