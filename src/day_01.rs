use std::collections::HashMap;

use crate::input_util;
use regex::Regex;

pub fn part_one(input_dir: &str) -> i64 {
    let first_digit = |x: &char| x.is_digit(10);
    let to_digit = |x: char| x.to_digit(10).expect("Already checked if it was a digit");

    input_util::read_file(&input_dir)
        .map(|line_read| {
            let line = line_read.expect("The input file is parsable");
            let line_characters = line.chars();
            // get first digit
            let first = line_characters
                .clone()
                .find(first_digit)
                .map(to_digit)
                .expect("Input line has at least one digit");
            // get last digit
            let last = line_characters
                .rev()
                .find(first_digit)
                .map(to_digit)
                .expect("Input line has at least one digit");
            // product of first and last digits
            return first as i64 * 10 + last as i64;
        })
        .sum()
}

pub fn find_first_digit(input: &str) -> u32 {
    let written_digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let written_digit_map = HashMap::from(written_digits);

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    if let Some(first) = re.find(input) {
        // println!("Input:\n{}\n{}", input, first.as_str());
        if let Some(replaced) = written_digit_map.get(first.as_str()) {
            return replaced.parse().expect("it is a digit");
        } else {
            return first.as_str().parse().expect("it is a digit");
        }
    }
    panic!("No digits found in input string");
}

pub fn find_last_digit(input: &str) -> u32 {
    let written_digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let written_digit_map = HashMap::from(written_digits);

    let re = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let (_line, captures) = re.captures(input).unwrap().extract::<1>();
    let first_capture = captures[0];
    // println!("Input:\n{}\n{}", input, first_capture);
    if let Some(first_digit) = written_digit_map.get(first_capture) {
        // first capture was a written out digit
        return first_digit.parse().expect("it is a digit");
    } else {
        // first capture is a digit
        return first_capture.parse().expect("it is a digit");
    }
}

pub fn part_two(input_dir: &str) -> i64 {
    input_util::read_file(&input_dir)
        .map(|line_read| {
            let line = line_read.expect("The input file is parsable");
            // get first digit
            let first = find_first_digit(&line);
            // get last digit
            let last = find_last_digit(&line);
            // product of first and last digits
            return first as i64 * 10 + last as i64;
        })
        .sum()
}
