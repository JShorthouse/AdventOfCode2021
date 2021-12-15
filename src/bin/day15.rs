use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use keyed_priority_queue::KeyedPriorityQueue;
use std::cmp;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

const OFFSETS: [Point; 4] = [
    Point{ x:  0, y: -1 },
    Point{ x:  1, y:  0 },
    Point{ x:  0, y:  1 },
    Point{ x: -1, y:  0 },
];

// Find path using Dijkstra's algorithm
fn shortest_path(weights: &Vec<Vec<u32>>) -> u32 {
    let grid_width = weights[0].len();
    let grid_height = weights.len();

    let mut distances: Vec<Vec<Option<u32>>> = Vec::new();
    for _ in 0..grid_height {
        distances.push( vec![None; grid_width] );
    }

    let goal = Point{ x: (grid_width-1) as i32, y: (grid_height-1) as i32 };

    let mut queue = KeyedPriorityQueue::<Point, cmp::Reverse<u32>>::new();
    queue.push( Point{ x: 0, y: 0 }, cmp::Reverse(0) );

    loop {
        let (cur_point, cmp::Reverse(cur_dist)) = queue.pop().unwrap();
        for offset in OFFSETS {
            let mut next_point = cur_point.clone();
            next_point.x += offset.x;
            next_point.y += offset.y;

            if next_point.x < 0 || next_point.y < 0
                || next_point.x >= weights[0].len() as i32
                || next_point.y >= weights.len() as i32
            {
                continue;
            }

            let next_x = usize::try_from(next_point.x).unwrap();
            let next_y = usize::try_from(next_point.y).unwrap();

            let next_dist = cur_dist + weights[next_y][next_x];

            if let Some(dist) = distances[next_y][next_x] {
                if next_dist > dist {
                    continue;
                }
            }

            if next_point == goal {
                return next_dist;
            }

            distances[next_y][next_x] = Some(next_dist);
            queue.push( next_point, cmp::Reverse(next_dist) );
        }
    }

}

fn main() {
    let file = File::open("input/15.txt").unwrap();
    let reader = BufReader::new(file);

    let mut weights: Vec<Vec<u32>> = Vec::new();
    let mut distances: Vec<Vec<Option<u32>>> = Vec::new();
    for line in reader.lines() {
        let vals: Vec<u32> = line.unwrap().chars()
                             .map(|c| c.to_string().parse::<u32>().unwrap() ).collect();
        distances.push(vec![None; vals.len()]);
        weights.push(vals);
    }

    let p1_ans = shortest_path(&weights);
    println!("Part 1: {}", p1_ans);

    let grid_width = weights[0].len();
    let grid_height = weights.len();
    let mut exp_grid: Vec<Vec<u32>> = Vec::new();

    for y in 0..grid_height*5 {
        exp_grid.push(Vec::new());
        for x in 0..grid_width*5 {
            let addition = (x/grid_width) + (y/grid_height);
            let value = (((weights[y%grid_height][x%grid_width] + addition as u32) -1 ) % 9) + 1;
            exp_grid[y].push(value);
        }
    }

    let p2_ans = shortest_path(&exp_grid);
    println!("Part 2: {}", p2_ans);
}
