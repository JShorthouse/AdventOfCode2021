use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use std::fs::read_to_string;

fn simulate(days: usize, timers: &mut [i64; 9]) -> i64 {
    for _ in 0..days {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }

    return timers.iter().sum();
}

fn main() {
    let mut input: Vec<i32> = read_to_string("input/06.txt").unwrap().trim()
                               .split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let mut timers = [0_i64; 9];

    for time in input {
        timers[time as usize] += 1;
    }

    println!("Part 1: {}", simulate(80,  &mut timers.clone()));
    println!("Part 2: {}", simulate(256, &mut timers.clone()));
}
