use std::collections::HashMap;

use regex::Regex;

use crate::{benchmark_util::Solution, input_util};

pub struct Day03;

#[derive(Clone)]
struct GearRatio {
    x: usize,
    y: usize,
    part: i64,
}

#[derive(Clone)]
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
            if higher.chars().any(|letter| '.'.ne(&letter)) {
                return true;
            }
        }
        let first = level.get(0).unwrap();
        let last = level.last().unwrap();
        if '.'.ne(first) && !first.is_numeric() || '.'.ne(last) && !last.is_numeric() {
            return true;
        }
        if let Some(lower) = below {
            let lower = &lower[range.clone()];
            if lower.chars().any(|letter| '.'.ne(&letter)) {
                return true;
            }
        }
        return false;
    }

    fn neighboring_gears(
        &self,
        above: Option<&String>,
        beside: &str,
        below: Option<&String>,
    ) -> Vec<GearRatio> {
        let length = beside.len();
        let range = if self.start == 0 { 0 } else { self.start - 1 }..(self.end + 1).min(length);

        let mut gears = vec![];
        let gear_pattern = Regex::new("B").unwrap();
        if let Some(above) = above {
            for gear_match in gear_pattern.find_iter(&above[range.clone()].replace('*', "B")) {
                gears.push(GearRatio {
                    x: gear_match.start() + range.start,
                    y: self.height - 1,
                    part: self.number,
                });
            }
        }
        for gear_match in gear_pattern.find_iter(&beside[range.clone()].replace('*', "B")) {
            gears.push(GearRatio {
                x: gear_match.start() + range.start,
                y: self.height,
                part: self.number,
            });
        }
        if let Some(below) = below {
            for gear_match in gear_pattern.find_iter(&below[range.clone()].replace('*', "B")) {
                gears.push(GearRatio {
                    x: gear_match.start() + range.start,
                    y: self.height + 1,
                    part: self.number,
                });
            }
        }
        // println!("Returning this many gears: {}", gears.len());
        return gears;
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
            .flat_map(|part| {
                part.neighboring_gears(
                    lines.get(part.height - 1),
                    lines.get(part.height).unwrap(),
                    lines.get(part.height + 1),
                )
            })
            .fold(
                HashMap::new(),
                |mut gear_ratios: HashMap<(usize, usize), Vec<i128>>, gear_ratio| {
                    if let Some(exists) = gear_ratios.get_mut(&(gear_ratio.x, gear_ratio.y)) {
                        exists.push(gear_ratio.part as i128);
                    } else {
                        gear_ratios
                            .insert((gear_ratio.x, gear_ratio.y), vec![gear_ratio.part as i128]);
                    }
                    gear_ratios
                },
            )
            .values()
            .filter(|list| list.len() > 1)
            .map(|list| list.iter().product::<i128>())
            .sum::<i128>() as i64
    }
}
