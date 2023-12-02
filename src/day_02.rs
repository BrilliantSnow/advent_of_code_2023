use std::collections::HashMap;

use crate::input_util;

pub fn part_one(input_dir: &str) -> i64 {
    let colors = [("red", 12), ("green", 13), ("blue", 14)];

    let color_map = HashMap::from(colors);

    input_util::read_file(&input_dir)
        .map(|line_read| {
            let line = line_read.expect("No issues reading file");
            let line_after_game = line.split_once(" ").unzip().1.unwrap();
            // let line_after_game = line.;
            let (game_number, data) = line_after_game.split_once(": ").unwrap();
            for round in data.split("; ") {
                for color_line in round.split(", ") {
                    let (amount, color) = color_line.split_once(" ").unwrap();
                    if let Some(max) = color_map.get(color) {
                        if amount.parse::<i32>().unwrap().cmp(max).is_gt() {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            return Some(game_number.parse::<i64>().unwrap());
        })
        .flatten()
        .sum()
}

pub fn part_two(input_dir: &str) -> i64 {
    todo!("part 2");
}
