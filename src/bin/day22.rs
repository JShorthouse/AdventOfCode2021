use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    min: Point,
    max: Point,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    state: State,
    cube: Cube,
}

fn intersection(cube1: &Cube, cube2: &Cube) -> Option<Cube> {
    let intersection = Cube{
        min: Point{
            x: std::cmp::max(cube1.min.x, cube2.min.x),
            y: std::cmp::max(cube1.min.y, cube2.min.y),
            z: std::cmp::max(cube1.min.z, cube2.min.z),
        },
        max: Point{
            x: std::cmp::min(cube1.max.x, cube2.max.x),
            y: std::cmp::min(cube1.max.y, cube2.max.y),
            z: std::cmp::min(cube1.max.z, cube2.max.z),
        }
    };

    if intersection.min.x > intersection.max.x ||
       intersection.min.y > intersection.max.y ||
       intersection.min.z > intersection.max.z
    {
        return None;
    }

    return Some(intersection);
}

// Split cube around intersection
// Make cubes by extending intersection faces outwards and then filling in surrounding space
// Potentially up to 6 splits
fn split_cube(cube: &Cube, intersection: &Cube) -> Vec<Cube> {
    let mut new_cubes = Vec::with_capacity(6);

    // Top and bottom
    // Also grow to fill x and z space as necessary
    if intersection.min.y != cube.min.y {
        new_cubes.push( Cube {
            min: Point {
                x: std::cmp::min(intersection.min.x, cube.min.x),
                y: cube.min.y,
                z: std::cmp::min(intersection.min.z, cube.min.z),
            },
            max: Point {
                x: std::cmp::max(intersection.max.x, cube.max.x),
                y: intersection.min.y - 1,
                z: std::cmp::max(intersection.max.z, cube.max.z),
            }
        });
    }
    if intersection.max.y != cube.max.y {
        new_cubes.push( Cube {
            min: Point {
                x: std::cmp::min(intersection.min.x, cube.min.x),
                y: intersection.max.y + 1,
                z: std::cmp::min(intersection.min.z, cube.min.z),
            },
            max: Point {
                x: std::cmp::max(intersection.max.x, cube.max.x),
                y: cube.max.y,
                z: std::cmp::max(intersection.max.z, cube.max.z),
            }
        });
    }

    // Left and right
    // Also grow to fill z space as necessary
    if intersection.min.x != cube.min.x {
        new_cubes.push( Cube {
            min: Point {
                x: cube.min.x,
                y: intersection.min.y,
                z: std::cmp::min(intersection.min.z, cube.min.z),
            },
            max: Point {
                x: intersection.min.x - 1,
                y: intersection.max.y,
                z: std::cmp::max(intersection.max.z, cube.max.z),
            }
        });
    }
    if intersection.max.x != cube.max.x {
        new_cubes.push( Cube {
            min: Point {
                x: intersection.max.x + 1,
                y: intersection.min.y,
                z: std::cmp::min(intersection.min.z, cube.min.z),
            },
            max: Point {
                x: cube.max.x,
                y: intersection.max.y,
                z: std::cmp::max(intersection.max.z, cube.max.z),
            }
        });
    }

    // Forward and backward
    // No extra growth needed as all excess space now covered
    if intersection.min.z != cube.min.z {
        new_cubes.push( Cube {
            min: Point {
                x: intersection.min.x,
                y: intersection.min.y,
                z: cube.min.z,
            },
            max: Point {
                x: intersection.max.x,
                y: intersection.max.y,
                z: intersection.min.z - 1,
            }
        });
    }
    if intersection.max.z != cube.max.z {
        new_cubes.push( Cube {
            min: Point {
                x: intersection.min.x,
                y: intersection.min.y,
                z: intersection.max.z + 1,
            },
            max: Point {
                x: intersection.max.x,
                y: intersection.max.y,
                z: cube.max.z,
            }
        });
    }

    return new_cubes;
}

fn total_area(cubes: &Vec<Cube>) -> i64 {
    let mut area: i64 = 0;
    for cube in cubes {
        area += (cube.max.x - cube.min.x + 1).abs() as i64
              * (cube.max.y - cube.min.y + 1).abs() as i64
              * (cube.max.z - cube.min.z + 1).abs() as i64;
    }

    return area;
}


fn main() {
    let file = File::open("input/22.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$").unwrap();

    let mut instructions = Vec::new();

    for input in reader.lines() {
        let input = input.unwrap();
        let captures = re.captures(&input).unwrap();

        let instruction = Instruction{
            state: match &captures[1] {
                "on" => State::On,
                "off" => State::Off,
                _ => unreachable!(),
            },
            cube: Cube {
                min: Point{
                    x: captures[2].parse::<i32>().unwrap(),
                    y: captures[4].parse::<i32>().unwrap(),
                    z: captures[6].parse::<i32>().unwrap(),
                },
                max: Point{
                    x: captures[3].parse::<i32>().unwrap(),
                    y: captures[5].parse::<i32>().unwrap(),
                    z: captures[7].parse::<i32>().unwrap(),
                },
            },
        };

        instructions.push(instruction);
    }

    instructions = instructions.into_iter().rev().collect();

    let mut cubes = Vec::new();
    let mut new_cubes = Vec::new();
    let mut p1_ans = None;

    loop {
        let ins = match instructions.pop() {
            Some(ins) => ins,
            None => break,
        };

        if p1_ans.is_none() {
            if ins.cube.min.x < -50 || ins.cube.min.y < -50 || ins.cube.min.z < -50
               || ins.cube.max.x > 50 || ins.cube.max.y > 50 || ins.cube.max.z > 50
            {
                p1_ans = Some(total_area(&cubes));
            }
        }

        let mut found_intersection = false;

        for other in &cubes {
            if !found_intersection {
                match intersection(&ins.cube, other) {
                    None => {
                        new_cubes.push(*other);
                    },
                    Some(inter) => {
                        new_cubes.append(&mut split_cube(other, &inter));

                        if ins.state == State::On {
                            new_cubes.push(inter.clone());
                        }

                        let remaining_parts = split_cube(&ins.cube, &inter);
                        for part in remaining_parts {
                            instructions.push( Instruction {
                                state: ins.state,
                                cube: part,
                            });
                        }

                        found_intersection = true;
                    }
                }
            } else {
                new_cubes.push(*other);
            }
        }

        if !found_intersection && ins.state == State::On {
            new_cubes.push(ins.cube);
        }

        std::mem::swap(&mut cubes, &mut new_cubes);
        new_cubes.clear();
    }

    println!("Part 1: {}", p1_ans.unwrap());

    let p2_ans = total_area(&cubes);
    println!("Part 2: {}", p2_ans);
}
