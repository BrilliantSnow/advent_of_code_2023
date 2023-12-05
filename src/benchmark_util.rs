use std::{
    ops::{Add, Div},
    time::{Duration, Instant},
};

use crate::{input_util, day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04};

pub fn run_benchmark(day_number: usize, iterations: usize) {
    let file_path = input_util::get_file_path(day_number);
    match day_number {
        1 => benchmark(Day01{}, &file_path, iterations),
        2 => benchmark(Day02{}, &file_path, iterations),
        3 => benchmark(Day03{}, &file_path, iterations),
        4 => benchmark(Day04{}, &file_path, iterations),
        _ => todo!(),
    }
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
