use regex::Regex;

use crate::{benchmark_util::Solution, input_util};

pub struct Day03;

struct PartNumber {
    start: usize,
    end: usize,
    height: usize,
    number: i64,
}

impl PartNumber {
    fn valid_part(&self, above: Option<&String>, beside: &str, below: Option<&String>) -> bool {
        let length = beside.len();
        let range = if self.start == 0 { 0 } else { self.start - 1 }..(self.end + 1).min(length - 1);
        let level: Vec<char> = beside[range.clone()].chars().collect();
        let mut adjacent_slices = vec![];
        let mut valid = false;
        if let Some(higher) = above {
            let higher = &higher[range.clone()];
            if higher
                .chars()
                .filter(|letter| '.'.ne(letter))
                .count()
                > 0
            {
                valid = true;
                // return true;
            }
            adjacent_slices.push(higher);
        }
        if '.'.ne(level.get(0).unwrap()) || '.'.ne(level.last().unwrap()) {
            valid = true;
            // return true;
        }
        adjacent_slices.push(&beside[range.clone()]);
        if let Some(lower) = below {
            let lower = &lower[range.clone()];
            if lower
                .chars()
                .filter(|letter| '.'.ne(letter))
                .count()
                > 0
            {
                valid = true;
                // return true;
            }
            adjacent_slices.push(lower);
        }
        if valid {
            println!("");
            for line in adjacent_slices {
                println!("{}", line);
            }
            return true;
        }
        return false;
    }
}

impl Solution for Day03 {
    fn part_one(&self, file_path: &str) -> i64 {
        let mut part_numbers = vec![];
        let number_pattern = Regex::new(r"(\d+)").unwrap();
        let lines: Vec<String> = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();
        for (index, line) in lines.iter().enumerate() {
            for find in number_pattern.find_iter(line) {
                part_numbers.push(PartNumber {
                    start: find.start(),
                    end: find.end(),
                    height: index,
                    number: find.as_str().parse().expect("Parsing a bunch of digits"),
                });
            }
        }
        part_numbers
            .iter()
            .filter(|part| {
                part.valid_part(
                    lines.get(part.height - 1),
                    lines.get(part.height).unwrap(),
                    lines.get(part.height + 1),
                )
            })
            .map(|part| part.number)
            .sum()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        todo!()
    }
}
