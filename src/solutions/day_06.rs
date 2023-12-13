use regex::Regex;

use crate::{benchmark_util::Solution, input_util};

pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, file_path: &str) -> i64 {
        let file_string: Vec<String> = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();

        let time_line = file_string.first().unwrap();
        let distance_line = file_string.get(1).unwrap();

        let number_pattern = Regex::new(r"\d+").unwrap();

        let times: Vec<i64> = number_pattern
            .find_iter(time_line)
            .map(|found| found.as_str().parse().unwrap())
            .collect();
        let distances: Vec<i64> = number_pattern
            .find_iter(distance_line)
            .map(|found| found.as_str().parse().unwrap())
            .collect();

        let find_distance =
            |total_time: i64, time_held_down: i64| (total_time - time_held_down) * time_held_down;

        (0..times.len())
            .map(|index| {
                let time = times[index];
                (0..time)
                    .filter(|&x| (find_distance)(time, x) > distances[index])
                    .count() as i64
            })
            .product()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        let file_string: Vec<String> = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();

        let time_line = file_string.first().unwrap();
        let distance_line = file_string.get(1).unwrap();

        let number_pattern = Regex::new(r"\d+").unwrap();

        let time: i64 = number_pattern
            .find_iter(time_line)
            .map(|found| found.as_str())
            .fold("".to_string(), |word, found| format!("{word}{found}"))
            .parse()
            .unwrap();
        let distance: i64 = number_pattern
            .find_iter(distance_line)
            .map(|found| found.as_str())
            .fold("".to_string(), |word, found| format!("{word}{found}"))
            .parse()
            .unwrap();

        let find_distance =
            |total_time: i64, time_held_down: i64| (total_time - time_held_down) * time_held_down;

        (0..time)
            .filter(|&x| (find_distance)(time, x) > distance)
            .count() as i64
    }
}
