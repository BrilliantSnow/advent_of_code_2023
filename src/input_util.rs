use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_file_path(day: usize) -> String {
    match day {
        1..=9 => format!("src/res/day_0{}.txt", day),
        10..=25 => format!("src/res/day_{}.txt", day),
        _ => panic!("Only designed for 25 days"),
    }
}

pub fn read_file_buffered(directory: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(directory).unwrap();
    let reader = BufReader::new(file);
    return reader.lines();
}
