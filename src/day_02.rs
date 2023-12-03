use std::collections::HashMap;

use crate::input_util;

pub fn part_one(input_dir: &str) -> i64 {
    let colors = [("red", 12), ("green", 13), ("blue", 14)];

    let color_map = HashMap::from(colors);

    input_util::read_file_buffered(&input_dir)
        .flatten()
        .zip(1..=100)
        .filter(|(line, _)| {
            line.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .flat_map(|instance| instance.split(", "))
                .map(|color_pair| color_pair.split_once(" ").unwrap())
                .all(|(amount, color)| {
                    color_map.get(color).map_or(false, |max| {
                        amount.parse::<usize>().unwrap().cmp(max).is_le()
                    })
                })
        })
        .map(|(_, index)| index)
        .sum()
}

pub fn part_two(input_dir: &str) -> i64 {
    todo!("part 2");
}
