use std::time::{Duration, Instant};

mod day_01;
mod day_02;
mod input_util;

fn run_day(day: usize) {
    match day {
        1 => {
            // day 1
            println!("Day One");
            let input = "src/res/day_01.txt";

            let output = day_01::part_one(input);

            println!("\t1: {}", output);

            let output = day_01::part_two(input);
            println!("\t2: {}", output);
        }
        2 => {
            // day 2
            println!("Day Two");
            let input = "src/res/day_02.txt";

            let run_times: Vec<Duration> = (0..10000)
                .map(|_| {
                    let now = Instant::now();
                    day_02::part_one(input);
                    return now.elapsed();
                })
                .collect();

            let best_time = run_times.iter().min().unwrap();

            let answer = day_02::part_one(input);
            println!("P1: {}", answer);
            println!("P1 duration {:.2?}", best_time);

            // println!("\t1: {}", output);

            let output = day_02::part_two(input);
            println!("\t2: {}", output);
        }
        _ => {
            // not implemented
            todo!();
        }
    }
}

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    run_day(2);
}
