use std::{
    ops::{Add, Div},
    time::{Duration, Instant},
};

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Solution {
    fn part_one(&self, file_path: &str) -> i64;

    fn part_two(&self, file_path: &str) -> i64;

    fn test(&self, file_path: &str, iterations: usize) {
        let run_times: Vec<Duration> = (0..iterations)
            .map(|_| {
                let now = Instant::now();
                self.part_one(file_path);
                return now.elapsed();
            })
            .collect();

        let average_time = run_times
            .iter()
            .fold(Duration::ZERO, |total, &x| total.add(x))
            .div(iterations as u32);

        let answer = self.part_one(file_path);

        println!("P1: {}", answer);
        println!("P1 duration {:.2?}", average_time);

        let run_times: Vec<Duration> = (0..iterations)
            .map(|_| {
                let now = Instant::now();
                self.part_two(file_path);
                return now.elapsed();
            })
            .collect();

        let average_time = run_times
            .iter()
            .fold(Duration::ZERO, |total, &x| total.add(x))
            .div(iterations as u32);

        let answer = self.part_two(file_path);

        println!("P2: {}", answer);
        println!("P2 duration {:.2?}", average_time);
    }
}
