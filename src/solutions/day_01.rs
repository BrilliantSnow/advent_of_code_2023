use std::collections::HashMap;

use crate::{benchmark_util::Solution, input_util};
use regex::Regex;

pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input_dir: &str) -> i64 {
        let first_digit = |x: &char| x.is_ascii_digit();
        let to_digit = |x: char| x.to_digit(10).expect("Already checked if it was a digit");

        input_util::read_file_buffered(input_dir)
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
                first as i64 * 10 + last as i64
            })
            .sum()
    }

    fn part_two(&self, input_dir: &str) -> i64 {
        input_util::read_file_buffered(input_dir)
            .map(|line_read| {
                let line = line_read.expect("The input file is parsable");
                Self::find_first_and_last_digits(&line)
            })
            .sum()
    }
}

impl Day01 {
    fn find_first_and_last_digits(input: &str) -> i64 {
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
        let first_match = re.find(input).expect("There to be a match");
        let first_digit = if let Some(digit_version) = written_digit_map.get(first_match.as_str()) {
            // first capture was a spelled digit
            digit_version
        } else {
            // first capture is already a digit
            first_match.as_str()
        };

        let re = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
        let (_line, captures) = re.captures(input).unwrap().extract::<1>();
        let first_capture = captures[0];
        let second_digit = if let Some(digit_version) = written_digit_map.get(first_capture) {
            // first capture was a spelled digit
            digit_version
        } else {
            // first capture is already a digit
            first_capture
        };

        format!("{}{}", first_digit, second_digit)
            .parse()
            .expect("To be a number")
    }
}
