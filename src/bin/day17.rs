use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Default)]
struct Bound {
    min: Point,
    max: Point,
}

fn main() {
    let input = read_to_string("input/17.txt").unwrap().trim().to_owned();
    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let captures = re.captures(&input).unwrap();

    let target = Bound{
                    min: Point{
                        x: captures[1].parse::<i32>().unwrap(),
                        y: captures[3].parse::<i32>().unwrap(),
                    },
                    max: Point {
                        x: captures[2].parse::<i32>().unwrap(),
                        y: captures[4].parse::<i32>().unwrap(),
                    },
                };

    let mut possible_y = Vec::new();

    // Find all y values that land in range
    for y_vel in target.min.y..=-target.min.y {
        // Optimization: for values >0 that arc up, velocity when returning to x=0 will be
        // start velocity + 1
        let mut cur_y_vel = if y_vel > 0 {
            -(y_vel+1)
        } else {
            y_vel
        };
        let mut y = 0;
        loop {
            if y < target.min.y {
                break;
            }

            y += cur_y_vel;
            cur_y_vel -= 1;

            if y >= target.min.y && y <= target.max.y {
                possible_y.push(y_vel);
                break;
            }
        }
    }

    let mut possible_x = Vec::new();

    // Find all x values that land in range
    for x_vel in 1..=target.max.x {
        let mut x = 0;
        let mut cur_x_vel = x_vel;
        loop {
            if cur_x_vel == 0 || x > target.max.x {
                break;
            }

            x += cur_x_vel;
            cur_x_vel -= 1;

            if x >= target.min.x && x <= target.max.x {
                possible_x.push(x_vel);
                break;
            }
        }
    }


    // Max height is sum of natural numbers of highest valid y (as velocity decreases by 1 each step)
    let max_y = possible_y[possible_y.len()-1];
    let max_height = ((max_y as f64 / 2.0) * (max_y as f64 + 1.0)) as i32;

    println!("Part 1: {}", max_height);

    let mut num_velocities = 0;
    // Simulate all x and y combinations to find which land within target
    for x_vel in &possible_x {
        for y_vel in &possible_y {
            let mut x = 0;
            let mut y = 0;
            let mut cur_x_vel = *x_vel;
            let mut cur_y_vel = *y_vel;

            loop {
                if x > target.max.x || y < target.min.y {
                    break;
                }

                x += cur_x_vel;
                y += cur_y_vel;

                cur_y_vel -= 1;
                if cur_x_vel > 0 {
                    cur_x_vel -= 1;
                }

                if x >= target.min.x && x <= target.max.x && y >= target.min.y && y <= target.max.y {
                    num_velocities += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {}", num_velocities);
}
