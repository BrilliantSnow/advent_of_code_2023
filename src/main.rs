use crate::{benchmark_util::Solution, day_01::Day01, day_02::Day02, day_03::Day03};

use day_04::Day04;
use enum_dispatch::enum_dispatch;

mod benchmark_util;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod input_util;

fn main() {
    println!("Welcome to my advent of code 2023 adventure!");
    let first = 1;
    let last = 4;
    for problem in first..=last {
        println!("======== Day {} =========", problem);
        run_day(solution(problem), problem);
    }
}

#[enum_dispatch(Solution)]
pub enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
}

fn solution(day: usize) -> Day {
    match day {
        1 => Day::Day01(Day01 {}),
        2 => Day::Day02(Day02 {}),
        3 => Day::Day03(Day03 {}),
        4 => Day::Day04(Day04 {}),
        _ => panic!("not implemented"),
    }
}

fn run_code<S:Fn(&str) -> i64>(part_1: S, file_path: &str) -> i64 {
    (part_1)(file_path)
}

fn run_day(day: Day, number: usize) {
    day.test(input_util::get_file_path(number).as_str(), 1);
}
