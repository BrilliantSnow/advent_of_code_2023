use std::time::Instant;

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

            let now = Instant::now();

            let output = day_02::part_one(input);

            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);

            println!("\t1: {}", output);

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
