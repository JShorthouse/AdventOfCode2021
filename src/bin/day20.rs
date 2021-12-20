use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use ahash::AHashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Field {
    points: AHashSet<Point>,
    void_state: bool,
}

const OFFSETS: [Point; 9] = [
    Point{ x: -1, y: -1 },
    Point{ x:  0, y: -1 },
    Point{ x:  1, y: -1 },
    Point{ x: -1, y:  0 },
    Point{ x:  0, y:  0 },
    Point{ x:  1, y:  0 },
    Point{ x: -1, y:  1 },
    Point{ x:  0, y:  1 },
    Point{ x:  1, y:  1 },
];

fn num_lit(field: &Field) -> usize {
    if field.void_state {
        panic!("Invalid state - infinite pixels lit");
    }
    field.points.len()
}

fn simulate(field: &mut Field, output_map: &Vec<bool>) {
    // Find bounds
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for point in &field.points {
        if point.x < min_x { min_x = point.x; }
        if point.y < min_y { min_y = point.y; }
        if point.x > max_x { max_x = point.x; }
        if point.y > max_y { max_y = point.y; }
    }

    min_x -= 1; max_x += 1; min_y -= 1; max_y += 1;

    let mut new_points = AHashSet::with_capacity(((field.points.len() as f32) * 1.2) as usize);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut value = 0;
            for offset in OFFSETS {
                let lit;
                let pos_x = x + offset.x;
                let pos_y = y + offset.y;

                // Points that are on the newly-expanded boundary or beyond are in the void
                if pos_x <= min_x || pos_x >= max_x || pos_y <= min_y || pos_y >= max_y {
                    lit = field.void_state;
                } else {
                    lit = field.points.contains( &Point{ x: pos_x, y: pos_y } );
                }

                value <<= 1;
                if lit {
                    value |= 1;
                }
            }
            if output_map[value] {
                new_points.insert( Point{ x, y } );
            }
        }
    }

    field.points = new_points;

    // Update void (will be either all 0's (0) or all 1's (511))
    field.void_state = match field.void_state {
        false => output_map[0],
        true => output_map[511],
    };
}

fn main() {
    let file = File::open("input/20.txt").unwrap();
    let mut reader = BufReader::new(file);

    let output_map: Vec<bool> = reader.by_ref().lines().next().unwrap().unwrap()
                                        .chars().map(|c| c == '#').collect();

    reader.by_ref().lines().next(); // Skip blank line

    let mut starting_points = AHashSet::<Point>::new();

    let mut y = 0;
    for line in reader.lines() {
        for (x, ch) in line.unwrap().chars().enumerate() {
            if ch == '#' {
                starting_points.insert(Point{x: x as i32, y});
            }
        }

        y += 1;
    }

    let mut field = Field{ points: starting_points, void_state: false };

    for _ in 0..2 {
        simulate(&mut field, &output_map);
    }
    println!("Part 1: {}", num_lit(&field));

    for _ in 0..48 {
        simulate(&mut field, &output_map);
    }
    println!("Part 2: {}", num_lit(&field));
}
