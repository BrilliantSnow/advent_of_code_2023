use std::{collections::HashMap, ops::Range};

use crate::{benchmark_util::Solution, input_util};

pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, file_path: &str) -> i64 {
        input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let numbers = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                let convert_to_numbers = |string_of_numbers: &str| {
                    string_of_numbers
                        .split_whitespace()
                        .into_iter()
                        .map(|digits| digits.parse().unwrap())
                        .collect::<Vec<i64>>()
                };
                let winning_numbers = convert_to_numbers(numbers.0);
                let my_numbers = convert_to_numbers(numbers.1);
                let matches = my_numbers
                    .iter()
                    .filter(|mine| winning_numbers.contains(mine))
                    .count() as u32;
                if matches == 0 {
                    0
                } else {
                    2i64.pow(matches - 1)
                }
            })
            .sum()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        const TOTAL: usize = 213;
        // list of what tickets each ticket gets copies of
        let free_ticket_lookup: [Range<usize>; TOTAL] = input_util::read_file_buffered(file_path)
            .flatten()
            .enumerate()
            .map(|(num, line)| {
                let numbers = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                let convert_to_numbers = |string_of_numbers: &str| {
                    string_of_numbers
                        .split_whitespace()
                        .into_iter()
                        .map(|digits| digits.parse().unwrap())
                        .collect::<Vec<i64>>()
                };
                let winning_numbers = convert_to_numbers(numbers.0);
                let my_numbers = convert_to_numbers(numbers.1);
                let count = my_numbers
                    .iter()
                    .filter(|mine| winning_numbers.contains(mine))
                    .count();
                (num..(num + count + 1).min(TOTAL), num)
            })
            // collect iterator into array
            .fold(std::array::from_fn(|_| 0..0), |mut arr, (range, num)| {
                arr[num] = range;
                arr
            });
        // keep track of how many tickets each ticket is responsible for multiplying
        let mut ticket_multipliers = [1; TOTAL];
        free_ticket_lookup
            .into_iter()
            .enumerate()
            .rev()
            .for_each(|(index, tickets)| {
                ticket_multipliers[index] *= ticket_multipliers[tickets].iter().sum::<i64>()
            });
        ticket_multipliers.iter().sum()
    }
}
