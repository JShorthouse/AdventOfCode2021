use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn main() {
    let file = File::open("input/01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut values = Vec::new();
    for line in reader.lines() {
       values.push( line.unwrap().parse::<i32>().unwrap() );
    }

    let mut p1: i32 = 0;
    for i in 1..values.len() {
        if values[i] > values[i-1] {
            p1 += 1;
        }
    }
    println!("Part 1: {}", p1);

    let mut windows = Vec::new();
    for i in 0..values.len()-2 {
        windows.push( values[i] + values[i+1] + values[i+2] );
    }

    let mut p2: i32 = 0;
    for i in 1..windows.len() {
        if windows[i] > windows[i-1] {
            p2 += 1;
        }
    }
    println!("Part 2: {}", p2);
}
