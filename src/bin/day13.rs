use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use std::convert::TryFrom;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn print_points(points: &HashSet<Point>) {
    let mut lower = Point{ x: i32::MAX, y: i32::MAX };
    let mut upper = Point{ x: i32::MIN, y: i32::MIN };

    for point in points {
        if point.x < lower.x { lower.x = point.x; }
        if point.y < lower.y { lower.y = point.y; }
        if point.x > upper.x { upper.x = point.x; }
        if point.y > upper.y { upper.y = point.y; }
    }

    let mut grid = Vec::new();
    let height = usize::try_from((upper.y-lower.y).abs()+1).unwrap();
    let width  = usize::try_from((upper.x-lower.x).abs()+1).unwrap();
    for _ in 0..height {
        grid.push( vec![false; width] )
    }
    for point in points {
        grid[usize::try_from(point.y-lower.y).unwrap()]
            [usize::try_from(point.x-lower.x).unwrap()] = true;
    }
    for row in grid {
        for val in row {
            match val {
                true  => print!("X"),
                false => print!(" "),
            }
        }
        print!("\n");
    }
}

fn main() {
    let file = File::open("input/13.txt").unwrap();
    let reader = BufReader::new(file);

    let mut processing_points = true;
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            processing_points = false;
            continue;
        }
        if processing_points {
            let split = line.split(",").collect::<Vec<&str>>();
            let p = Point {
                x: split[0].parse::<i32>().unwrap(),
                y: split[1].parse::<i32>().unwrap(),
            };
            points.insert(p);
        } else {
            let line = &line["fold along ".len()..];
            let split = line.split("=").collect::<Vec<&str>>();
            let fold = match split[0] {
                "x" => Fold::X(split[1].parse::<i32>().unwrap()),
                "y" => Fold::Y(split[1].parse::<i32>().unwrap()),
                _ => unreachable!(),
            };
            folds.push(fold);
        }
    }

    let mut p1_ans = 0;

    for (idx, fold) in folds.iter().enumerate() {
        let mut new_points = HashSet::new();
        for point in points {
            let mut point = point.clone();
            match fold {
                Fold::X(fold_x) => {
                    if point.x > *fold_x {
                        point.x = fold_x - (point.x - fold_x);
                    }
                },
                Fold::Y(fold_y) => {
                    if point.y > *fold_y {
                        point.y = fold_y - (point.y - fold_y);
                    }
                }
            }
            new_points.insert(point);
        }
        points = new_points;
        if idx == 0 {
            p1_ans = points.len();
        }
    }

    println!("Part 1: {}", p1_ans);
    println!("Part 2:");
    print_points(&points);
}
