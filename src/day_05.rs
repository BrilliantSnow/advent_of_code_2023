use std::{collections::HashMap, ops::Range};

use regex::Regex;

use crate::{benchmark_util::Solution, input_util};

pub struct Day05;

type Mapping = (Range<i64>, Range<i64>, i64);

struct Mapper {
    conversion: Vec<Mapping>,
}

impl Mapper {
    pub fn map(&self, input: i64) -> i64 {
        let oof: Vec<i64> = self
            .conversion
            .iter()
            .filter(|(from, to, offset)| from.contains(&input))
            // .find(|(from, to, offset)| from.contains(&input))
            .map(|(from, to, offset)| input + offset)
            .collect();
        // .unwrap_or(input)
        if oof.len() > 1 {
            println!("OOF");
            todo!();
        } else if oof.len() == 1 {
            return *oof.first().unwrap();
        } else {
            return input;
        }
    }

    pub fn map_backwards(&self, input: i64) -> i64 {
        self.conversion
            .iter()
            .find(|(from, to, offset)| to.contains(&input))
            // .find(|(from, to, offset)| from.contains(&input))
            .map(|(from, to, offset)| input - offset)
            .unwrap_or(input)
        // if oof.len() > 1 {
        //     println!("OOF");
        //     todo!();
        // } else if oof.len() == 1 {
        //     return *oof.first().unwrap();
        // } else {
        //     return input;
        // }
    }
}

impl Solution for Day05 {
    fn part_one(&self, file_path: &str) -> i64 {
        let seeds_pattern = Regex::new(r"seeds:(:? \d+)+").unwrap();

        let map_pattern = Regex::new(r"\w+-to-\w+ map:\n(\d+[ \n])+").unwrap();

        let map_header_pattern = Regex::new(r"(\w+)-to-(\w+)").unwrap();
        let map_data_pattern = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

        let file_string: String = input_util::read_file_buffered(file_path)
            .flatten()
            .fold("".into(), |file, line| format!("{}\n{}", file, line));

        let seed_line = seeds_pattern
            .find(&file_string)
            .expect("seeds to exist in file")
            .as_str();

        let seeds: Vec<i64> = seed_line
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .into_iter()
            .map(|digits| digits.parse().unwrap())
            .collect();

        let mut range_mapper: HashMap<(&str, &str), Mapper> = HashMap::default();

        for map_match in map_pattern.find_iter(&file_string) {
            let (_, captures) = map_header_pattern
                .captures(map_match.as_str())
                .unwrap()
                .extract::<2>();
            let key = (captures[0], captures[1]);
            let values: Vec<Mapping> = map_data_pattern
                .captures_iter(map_match.as_str())
                .map(|captures| captures.extract::<3>())
                .map(|(_, captures)| {
                    let to: i64 = captures[0].parse().unwrap();
                    let from: i64 = captures[1].parse().unwrap();
                    let length: i64 = captures[2].parse().unwrap();
                    (from..from + length, to..to + length, to - from)
                })
                .collect();
            range_mapper.insert(key, Mapper { conversion: values });
        }

        // println!("My maps contents: {:?}", range_mapper.keys());

        let conversions_needed = vec![
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];

        seeds
            .into_iter()
            .map(|seed_number| {
                let mut id = seed_number;
                let mut count = 0;
                for conversion in conversions_needed.windows(2) {
                    let from = conversion.get(0).unwrap();
                    let to = conversion.get(1).unwrap();
                    // println!(
                    //     "Trying to convert {} to {}",
                    //     conversions_needed[count],
                    //     conversions_needed[count + 1]
                    // );
                    let old_id = id;
                    id = range_mapper
                        .get(&(*from, *to))
                        .expect("Why no conversion")
                        .map(seed_number);
                    // println!("{} to {}", old_id, id);
                    count += 1;
                }
                return id;
            })
            .min()
            .unwrap()
    }

    fn part_two(&self, file_path: &str) -> i64 {
        let seeds_pattern = Regex::new(r"seeds:(:? \d+)+").unwrap();

        let map_pattern = Regex::new(r"\w+-to-\w+ map:\n(\d+[ \n])+").unwrap();

        let map_header_pattern = Regex::new(r"(\w+)-to-(\w+)").unwrap();
        let map_data_pattern = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

        let seed_range_pattern = Regex::new(r"(\d+) (\d+)").unwrap();

        let file_string: String = input_util::read_file_buffered(file_path)
            .flatten()
            .fold("".into(), |file, line| format!("{}\n{}", file, line));

        let seed_line = seeds_pattern
            .find(&file_string)
            .expect("seeds to exist in file")
            .as_str();

        let seeds: Vec<Range<i64>> = seed_range_pattern
            .captures_iter(seed_line.split_once(": ").unwrap().1)
            .map(|captures| captures.extract::<2>())
            .into_iter()
            .map(|(_, [left, right])| {
                let lower = left.parse().unwrap();
                let length: i64 = right.parse().unwrap();
                return lower..lower + length;
            })
            .collect();

        let mut range_mapper: HashMap<(&str, &str), Mapper> = HashMap::default();

        for map_match in map_pattern.find_iter(&file_string) {
            let (_, captures) = map_header_pattern
                .captures(map_match.as_str())
                .unwrap()
                .extract::<2>();
            let key = (captures[0], captures[1]);
            let values: Vec<Mapping> = map_data_pattern
                .captures_iter(map_match.as_str())
                .map(|captures| captures.extract::<3>())
                .map(|(_, captures)| {
                    let to: i64 = captures[0].parse().unwrap();
                    let from: i64 = captures[1].parse().unwrap();
                    let length: i64 = captures[2].parse().unwrap();
                    (from..from + length, to..to + length, to - from)
                })
                .collect();
            range_mapper.insert(key, Mapper { conversion: values });
        }

        // println!("My maps contents: {:?}", range_mapper.keys());

        let conversions_needed = vec![
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];

        (0..i64::MAX)
            .find(|&location_id| {
                let mut id = location_id;
                conversions_needed.windows(2).rev().for_each(|pair| {
                    let [from, to]: [&str; 2] = pair.try_into().unwrap();
                    id = range_mapper.get(&(from, to)).unwrap().map_backwards(id);
                });
                seeds.iter().any(|seed_list| seed_list.contains(&id))
            }
            )
            .expect("Not to run out of locations")

        // seeds
        //     .into_iter()
        //     .map(|seed_range: Range<i64>| {
        //         seed_range
        //             .into_iter()
        //             .map(|seed_number| {
        //                 let mut id = seed_number;
        //                 for conversion in conversions_needed.windows(2) {
        //                     let from = conversion.get(0).unwrap();
        //                     let to = conversion.get(1).unwrap();
        //                     id = range_mapper
        //                         .get(&(*from, *to))
        //                         .expect("Why no conversion")
        //                         .map(seed_number);
        //                 }
        //                 return id;
        //             })
        //             .min()
        //     })
        //     .flatten()
        //     .min()
        //     .unwrap()
    }
}
