use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::convert::TryFrom;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: isize,
    y: isize,
}

const OFFSETS: [Point; 8] = [
    Point{ x: -1, y: -1 },
    Point{ x:  0, y: -1 },
    Point{ x:  1, y: -1 },
    Point{ x: -1, y:  0 },
    Point{ x:  1, y:  0 },
    Point{ x: -1, y:  1 },
    Point{ x:  0, y:  1 },
    Point{ x:  1, y:  1 },
];

fn main() {
    let file = File::open("input/11.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        grid.push( line.unwrap().chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect() );
    }

    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let flash_target = grid_height * grid_width;

    let mut p1_ans = 0;
    let p2_ans;
    let mut total_flashes = 0;
    let mut i = 0;

    loop {
        i += 1;
        // Increment all
        for x in 0..grid_width {
            for y in 0..grid_height {
                grid[y][x] += 1;
            }
        }

        // Simulate flashes
        let mut flash_count = 0;
        loop {
            let mut flashed = false;
            for x in 0..grid_width {
                for y in 0..grid_height {
                    if grid[y][x] > 9 {
                        flashed = true;
                        flash_count += 1;
                        grid[y][x] = i32::MIN;

                        for offset in OFFSETS {
                            let off_y = y as isize + offset.y;
                            let off_x = x as isize + offset.x;
                            if off_y < 0 || off_y >= grid_height as isize || off_x < 0 || off_x >= grid_width as isize {
                                continue;
                            }
                            grid[usize::try_from(off_y).unwrap()][usize::try_from(off_x).unwrap()] += 1;
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }

        // Reset flashed
        for x in 0..grid_width {
            for y in 0..grid_height {
                if grid[y][x] < 0 {
                    grid[y][x] = 0;
                }
            }
        }

        total_flashes += flash_count;
        if i == 100 {
            p1_ans = total_flashes;
        }
        if flash_count == flash_target {
            p2_ans = i;
            break;
        }
    }

    println!("Part 1: {}", p1_ans);
    println!("Part 2: {}", p2_ans);
}
