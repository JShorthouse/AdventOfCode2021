use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

use std::collections::HashSet;

fn main() {
    let file = File::open("input/09.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        grid.push( line.unwrap().chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect() );
    }

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut low_points = Vec::<Point>::new();

    for x in 0..grid_width {
        for y in 0..grid_height {
            let cur_val = grid[y][x];
            if x >= 1 && grid[y][x-1] <= cur_val {
                continue;
            }
            if x < grid_width-1 && grid[y][x+1] <= cur_val {
                continue;
            }
            if y >= 1 && grid[y-1][x] <= cur_val {
                continue;
            }
            if y < grid_height-1 && grid[y+1][x] <= cur_val {
                continue;
            }

            low_points.push(Point{x,y});
        }
    }

    let mut risk_level = 0;
    for point in &low_points {
        risk_level += grid[point.y][point.x] + 1;
    }

    println!("Part 1: {}", risk_level);

    let mut basin_sizes = Vec::<usize>::new();

    for low_point in low_points {
        let mut basin_points = HashSet::<Point>::new();
        let mut edges = Vec::<Point>::new();
        basin_points.insert(low_point.clone());
        edges.push(low_point);

        let mut new_edges = Vec::<Point>::new();
        let mut found_new = true;
        while found_new {
            found_new = false;
            new_edges.clear();

            for edge in &edges {
                let x = edge.x;
                let y = edge.y;
                if x >= 1 && grid[y][x-1] < 9 {
                    new_edges.push(Point{y: y, x: x-1});
                }
                if x < grid_width-1 && grid[y][x+1] < 9 {
                    new_edges.push(Point{y: y, x: x+1});
                }
                if y >= 1 && grid[y-1][x] < 9 {
                    new_edges.push(Point{y: y-1, x: x});
                }
                if y < grid_height-1 && grid[y+1][x] < 9 {
                    new_edges.push(Point{y: y+1, x: x});
                }
            }
            edges.clear();
            for edge in &new_edges {
                if !basin_points.contains(&edge) {
                    found_new = true;
                    basin_points.insert(edge.clone());
                    edges.push(*edge);
                }
            }
        }
        basin_sizes.push(basin_points.len());
    }

    basin_sizes.sort();
    let p2_ans = basin_sizes[basin_sizes.len()-1] * basin_sizes[basin_sizes.len()-2]
                     * basin_sizes[basin_sizes.len()-3];

    println!("Part 2: {}", p2_ans);
}
