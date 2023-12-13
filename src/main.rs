use benchmark_util::run_benchmark;

use crate::benchmark_util::run_day;

mod benchmark_util;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod input_util;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    let first = 10;
    let last = 10;
    for day_number in first..=last {
        println!("======== Day {} =========", day_number);
        // run_benchmark(day_number, 100);
        run_day(day_number);
    }
}
