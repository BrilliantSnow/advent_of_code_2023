use benchmark_util::run_benchmark;

use crate::benchmark_util::run_day;

mod benchmark_util;
mod solutions;

mod input_util;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    let first = 1;
    let last = 11;
    for day_number in first..=last {
        println!("======== Day {} =========", day_number);
        run_benchmark(day_number, 100);
        run_day(day_number);
    }
}
