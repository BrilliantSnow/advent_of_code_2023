use crate::input_util;

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
