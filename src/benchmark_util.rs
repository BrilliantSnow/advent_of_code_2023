use std::{
    ops::{Add, Div},
    time::{Duration, Instant},
};

use crate::{input_util, solutions};

pub fn run_benchmark(day_number: usize, iterations: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => benchmark(solutions::day_01::Day01 {}, &file_path, iterations),
        2 => benchmark(solutions::day_02::Day02 {}, &file_path, iterations),
        3 => benchmark(solutions::day_03::Day03 {}, &file_path, iterations),
        4 => benchmark(solutions::day_04::Day04 {}, &file_path, iterations),
        5 => benchmark(solutions::day_05::Day05 {}, &file_path, iterations),
        6 => benchmark(solutions::day_06::Day06 {}, &file_path, iterations),
        7 => benchmark(solutions::day_07::Day07 {}, &file_path, iterations),
        8 => benchmark(solutions::day_08::Day08 {}, &file_path, iterations),
        9 => benchmark(solutions::day_09::Day09 {}, &file_path, iterations),
        10 => benchmark(solutions::day_10::Day10 {}, &file_path, iterations),
        11 => benchmark(solutions::day_11::Day11 {}, &file_path, iterations),
        12 => benchmark(solutions::day_12::Day12 {}, &file_path, iterations),
        13 => benchmark(solutions::day_13::Day13 {}, &file_path, iterations),
        _ => todo!(),
    }
}

pub fn run_day(day_number: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => run(solutions::day_01::Day01 {}, &file_path),
        2 => run(solutions::day_02::Day02 {}, &file_path),
        3 => run(solutions::day_03::Day03 {}, &file_path),
        4 => run(solutions::day_04::Day04 {}, &file_path),
        5 => run(solutions::day_05::Day05 {}, &file_path),
        6 => run(solutions::day_06::Day06 {}, &file_path),
        7 => run(solutions::day_07::Day07 {}, &file_path),
        8 => run(solutions::day_08::Day08 {}, &file_path),
        9 => run(solutions::day_09::Day09 {}, &file_path),
        10 => run(solutions::day_10::Day10 {}, &file_path),
        11 => run(solutions::day_11::Day11 {}, &file_path),
        12 => run(solutions::day_12::Day12 {}, &file_path),
        13 => run(solutions::day_13::Day13 {}, &file_path),
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
            now.elapsed()
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
            now.elapsed()
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
