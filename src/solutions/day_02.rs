use std::collections::HashMap;

use crate::{benchmark_util::Solution, input_util};

pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input_dir: &str) -> i64 {
        let colors = [("red", 12), ("green", 13), ("blue", 14)];

        let color_map = HashMap::from(colors);

        input_util::read_file_buffered(input_dir)
            .flatten()
            .zip(1..=100)
            .filter(|(line, _)| {
                line.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .flat_map(|instance| instance.split(", "))
                    .map(|color_pair| color_pair.split_once(' ').unwrap())
                    .all(|(amount, color)| {
                        color_map.get(color).map_or(false, |max| {
                            amount.parse::<usize>().unwrap().cmp(max).is_le()
                        })
                    })
            })
            .map(|(_, index)| index)
            .sum()
    }

    fn part_two(&self, input_dir: &str) -> i64 {
        input_util::read_file_buffered(input_dir)
            .flatten()
            .map(|line| {
                line.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .flat_map(|instance| instance.split(", "))
                    .map(|color_pair| color_pair.split_once(' ').unwrap())
                    .fold(HashMap::new(), |mut color_map, (amount_str, color)| {
                        let amount: i64 = amount_str.parse().unwrap();
                        if color_map
                            .get(color)
                            .filter(|existing_amount| amount.cmp(existing_amount).is_le())
                            .is_none()
                        {
                            color_map.insert(color, amount);
                        }
                        color_map
                    })
                    .values()
                    .product::<i64>()
            })
            .sum()
    }
}
