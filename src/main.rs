use benchmark_util::run_benchmark;

use crate::benchmark_util::run_day;

mod benchmark_util;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod input_util;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    let first = 6;
    let last = 6;
    for day_number in first..=last {
        println!("======== Day {} =========", day_number);
        // run_benchmark(day_number, );
        run_day(day_number);
    }
}
