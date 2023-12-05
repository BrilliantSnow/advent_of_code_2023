use benchmark_util::run_benchmark;

mod benchmark_util;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod input_util;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    let first = 4;
    let last = 4;
    for day_number in first..=last {
        println!("======== Day {} =========", day_number);
        run_benchmark(day_number, 1000);
    }
}
