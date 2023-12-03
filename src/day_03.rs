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

        let range = if self.start == 0 { 0 } else { self.start - 1 }..(self.end + 1).min(length);
        let level: Vec<char> = beside[range.clone()].chars().collect();
        if let Some(higher) = above {
            let higher = &higher[range.clone()];
            if higher
                .chars()
                .filter(|letter| '.'.ne(letter))
                .filter(|letter| !letter.is_numeric())
                .count()
                > 0
            {
                return true;
            }
        }
        let first = level.get(0).unwrap();
        let last = level.last().unwrap();
        let first_is_symbol = '.'.ne(first) && !first.is_numeric();
        let second_is_symbol = '.'.ne(last) && !last.is_numeric();
        if first_is_symbol || second_is_symbol {
            return true;
        }
        if let Some(lower) = below {
            let lower = &lower[range.clone()];
            if lower
                .chars()
                .filter(|letter| '.'.ne(letter))
                .filter(|letter| !letter.is_numeric())
                .count()
                > 0
            {
                return true;
            }
        }
        return false;
    }
}

impl Solution for Day03 {
    fn part_one(&self, file_path: &str) -> i64 {
        let number_pattern = Regex::new(r"(\d+)").unwrap();
        let lines: Vec<String> = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();
        lines
            .iter()
            .enumerate()
            .flat_map(|(index, line)| {
                number_pattern.find_iter(&line).map(move |find| PartNumber {
                    start: find.start(),
                    end: find.end(),
                    height: index,
                    number: find.as_str().parse().expect("Parsing a bunch of digits"),
                })
            })
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
