use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use std::collections::HashMap;

fn main() {
    let file = File::open("input/14.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut sequence: Vec<char> = reader.by_ref().lines().next().unwrap().unwrap()
                                        .chars().collect();
    reader.by_ref().lines().next(); // Skip whitespace

    let mut rules = HashMap::<String, char>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let key = line[0..2].to_owned();
        let value = line.chars().nth(6).unwrap();
        rules.insert(key, value);
    }

    for _iteration in 0..10 {
        let mut new_sequence = Vec::with_capacity((sequence.len() as f64 *1.5) as usize);

        for (idx, ch) in sequence.iter().enumerate() {
            if idx == sequence.len()-1 {
                new_sequence.push(*ch);
            } else {
                let key: String = sequence[idx..=idx+1].iter().collect();
                let new = rules[&key];
                new_sequence.push(*ch);
                new_sequence.push(new);
            }
        }

        sequence = new_sequence;
    }

    let mut counts = HashMap::<char, i64>::new();
    for ch in sequence {
        let entry = counts.entry(ch).or_insert(0);
        *entry += 1;
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for (_ch, count) in counts {
        if count < min {
            min = count;
        }
        if count > max {
            max = count;
        }
    }

    println!("Part 1: {}", max - min);
}
