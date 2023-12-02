mod input_util;
mod day_01;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    // day 1
    println!("Day One");
    let day_one_part_one = day_01::part_one("src/res/day_01.txt");
    println!("\t1: {}", day_one_part_one);
}
