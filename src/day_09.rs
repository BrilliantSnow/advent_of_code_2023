use crate::{benchmark_util::Solution, input_util};

pub struct Day09;

impl Solution for Day09 {
    fn part_one(&self, file_path: &str) -> i64 {
        input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let numbers: Vec<i64> = line
                    .split(" ")
                    .map(|number_string| number_string.parse().unwrap())
                    .collect();
                let mut degrees: Vec<Vec<i64>> = vec![numbers];
                loop {
                    if degrees.last().unwrap().iter().all(|x| 0.eq(x)) {
                        break;
                    }
                    degrees.push(
                        degrees
                            .last()
                            .unwrap()
                            .windows(2)
                            .map(|pair| (pair[0], pair[1]))
                            .map(|(left, right)| right - left)
                            .collect(),
                    );
                }
                let last_digits: Vec<i64> = degrees
                    .into_iter()
                    .map(|list| *list.iter().last().unwrap())
                    .collect();
                let mut cursor = None;
                for digit in last_digits.into_iter().rev() {
                    cursor = cursor.map(|x| x + digit).or(Some(digit));
                }
                cursor
            })
            .flatten()
            .sum()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        input_util::read_file_buffered(file_path)
            .flatten()
            .map(|line| {
                let numbers: Vec<i64> = line
                    .split(" ")
                    .map(|number_string| number_string.parse().unwrap())
                    .collect();

                let mut degrees: Vec<Vec<i64>> = vec![numbers];
                loop {
                    if degrees.last().unwrap().iter().all(|x| 0.eq(x)) {
                        break;
                    }
                    degrees.push(
                        degrees
                            .last()
                            .unwrap()
                            .windows(2)
                            .map(|pair| (pair[0], pair[1]))
                            .map(|(left, right)| right - left)
                            .collect(),
                    );
                }
                let last_digits: Vec<i64> = degrees
                    .into_iter()
                    .map(|list| *list.first().unwrap())
                    .collect();
                let mut cursor = None;
                for digit in last_digits.into_iter().rev() {
                    cursor = cursor.map(|x| digit - x).or(Some(digit));
                }
                cursor
            })
            .flatten()
            .sum()
    }
}
