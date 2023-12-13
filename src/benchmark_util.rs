use std::{
    ops::{Add, Div},
    time::{Duration, Instant},
};

use crate::{
    day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04, day_05::Day05, day_06::Day06,
    day_07::Day07, day_08::Day08, day_09::Day09, day_10::Day10, input_util,
};

pub fn run_benchmark(day_number: usize, iterations: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => benchmark(Day01 {}, &file_path, iterations),
        2 => benchmark(Day02 {}, &file_path, iterations),
        3 => benchmark(Day03 {}, &file_path, iterations),
        4 => benchmark(Day04 {}, &file_path, iterations),
        5 => benchmark(Day05 {}, &file_path, iterations),
        6 => benchmark(Day06 {}, &file_path, iterations),
        7 => benchmark(Day07 {}, &file_path, iterations),
        8 => benchmark(Day08 {}, &file_path, iterations),
        9 => benchmark(Day09 {}, &file_path, iterations),
        10 => benchmark(Day10 {}, &file_path, iterations),
        _ => todo!(),
    }
}

pub fn run_day(day_number: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => run(Day01 {}, &file_path),
        2 => run(Day02 {}, &file_path),
        3 => run(Day03 {}, &file_path),
        4 => run(Day04 {}, &file_path),
        5 => run(Day05 {}, &file_path),
        6 => run(Day06 {}, &file_path),
        7 => run(Day07 {}, &file_path),
        8 => run(Day08 {}, &file_path),
        9 => run(Day09 {}, &file_path),
        10 => run(Day10 {}, &file_path),
        _ => todo!(),
    }
}

pub fn run<S: Solution>(day_solution: S, file_path: &str) {
    let answer = day_solution.part_one(file_path);

    println!("P1: {}", answer);

    let answer = day_solution.part_two(file_path);

    println!("P2: {}", answer);
}

fn benchmark<S: Solution>(day_solution: S, file_path: &str, iterations: usize) {
    let run_times: Vec<Duration> = (0..iterations)
        .map(|_| {
            let now = Instant::now();
            day_solution.part_one(file_path);
            return now.elapsed();
        })
        .collect();

    let average_time = run_times
        .iter()
        .fold(Duration::ZERO, |total, &x| total.add(x))
        .div(iterations as u32);

    let answer = day_solution.part_one(file_path);

    println!("P1: {}", answer);
    println!("P1 duration {:.2?}", average_time);

    let run_times: Vec<Duration> = (0..iterations)
        .map(|_| {
            let now = Instant::now();
            day_solution.part_two(file_path);
            return now.elapsed();
        })
        .collect();

    let average_time = run_times
        .iter()
        .fold(Duration::ZERO, |total, &x| total.add(x))
        .div(iterations as u32);

    let answer = day_solution.part_two(file_path);

    println!("P2: {}", answer);
    println!("P2 duration {:.2?}", average_time);
}

pub trait Solution {
    fn part_one(&self, file_path: &str) -> i64;

    fn part_two(&self, file_path: &str) -> i64;
}
