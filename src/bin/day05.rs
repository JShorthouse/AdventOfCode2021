use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

use ahash::AHashMap;

fn count_overlaps(lines: &Vec<Line>) -> i32 {
    let mut pos_counts = AHashMap::<(i32, i32), i32>::with_capacity(100000);

    for line in lines {
        let mut x = line.start.x;
        let mut y = line.start.y;

        let mut increment = [0, 0];
        if line.start.x != line.end.x {
            increment[0] = if line.start.x < line.end.x { 1 } else { -1 };
        }
        if line.start.y != line.end.y {
            increment[1] = if line.start.y < line.end.y { 1 } else { -1 };
        }

        loop {
            let count = pos_counts.entry((x,y)).or_insert(0);
            *count += 1;

            if x == line.end.x && y == line.end.y {
                break;
            }
            x += increment[0];
            y += increment[1];
        }
    }

    let mut count = 0;
    for (key, val) in &pos_counts {
        if *val > 1 { count += 1; }
    }
    return count;
}

fn main() {
    let file = File::open("input/05.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    let mut lines = Vec::new();
    for input in reader.lines() {
        let input = input.unwrap();
        let captures = re.captures(&input).unwrap();

        lines.push(
            Line{
                start: Point {
                    x: captures[1].parse::<i32>().unwrap(),
                    y: captures[2].parse::<i32>().unwrap(),
                },
                end:   Point {
                    x: captures[3].parse::<i32>().unwrap(),
                    y: captures[4].parse::<i32>().unwrap(),
                },
            }
        );
    }

    let axial_lines = lines.iter().cloned().filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
                      .collect::<Vec<Line>>();

    println!("Part 1: {}", count_overlaps(&axial_lines));
    println!("Part 2: {}", count_overlaps(&lines));
}
