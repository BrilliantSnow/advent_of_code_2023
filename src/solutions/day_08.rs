use std::collections::HashMap;

use regex::Regex;

use crate::{benchmark_util::Solution, input_util};

pub struct Day08;

impl Solution for Day08 {
    fn part_one(&self, file_path: &str) -> i64 {
        let input_string: String = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();
        let step_pattern = Regex::new(r"[LR]+").unwrap();
        let node_pattern = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

        let steps = step_pattern.find(&input_string).unwrap().as_str();
        let nodes: HashMap<&str, (&str, &str)> = node_pattern
            .captures_iter(&input_string)
            .map(|captures| {
                let (_, [key, left, right]) = captures.extract::<3>();
                (key, (left, right))
            })
            .collect();

        let mut current_step = "AAA";
        for (count, letter) in steps.chars().cycle().enumerate() {
            if current_step == "ZZZ" {
                return count as i64;
            }
            current_step = match letter {
                'L' => nodes.get(current_step).unwrap().0,
                'R' => nodes.get(current_step).unwrap().1,
                _ => unreachable!("String is only L and R"),
            };
        }
        unreachable!("Steps has at least 1 letter");
    }

    fn part_two(&self, file_path: &str) -> i64 {
        let input_string: String = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();
        let step_pattern = Regex::new(r"[LR]+").unwrap();
        let node_pattern = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

        let steps = step_pattern.find(&input_string).unwrap().as_str();
        let nodes: HashMap<&str, (&str, &str)> = node_pattern
            .captures_iter(&input_string)
            .map(|captures| {
                let (_, [key, left, right]) = captures.extract::<3>();
                (key, (left, right))
            })
            .collect();

        nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|&start| {
                let mut cursor = start;
                for (index, direction) in steps.chars().cycle().enumerate() {
                    if cursor.ends_with('Z') {
                        return index as i64;
                    }
                    cursor = match direction {
                        'L' => nodes.get(cursor).unwrap().0,
                        'R' => nodes.get(cursor).unwrap().1,
                        _ => unreachable!("Only left and right steps"),
                    };
                }
                unreachable!();
            })
            .fold(1, num::integer::lcm)
    }
}
