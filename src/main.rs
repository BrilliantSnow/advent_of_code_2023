mod input_util;
mod day_01;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    // day 1
    println!("Day One");
    let input = "src/res/day_01.txt";

    let output = day_01::part_one(input);
    println!("\t1: {}", output);

    let output = day_01::part_two(input);
    println!("\t2: {}", output);
}
