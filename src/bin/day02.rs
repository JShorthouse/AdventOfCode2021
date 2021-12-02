use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

enum Dir {
    Forward,
    Down,
    Up,
}

struct Command {
    dir: Dir,
    amount: i32,
}

fn main() {
    let file = File::open("input/02.txt").unwrap();
    let reader = BufReader::new(file);

    let mut commands = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" ").collect();
        let dir = match split[0] {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => unreachable!(),
        };
        let amount = split[1].parse::<i32>().unwrap();

        commands.push( Command{ dir, amount} )
    }

    let mut vert = 0;
    let mut hori = 0;

    for command in &commands {
        match command.dir {
            Dir::Forward => { hori += command.amount },
            Dir::Down =>    { vert += command.amount },
            Dir::Up =>      { vert -= command.amount },
        }
    }
    println!("Part 1: {}", vert * hori);

    vert = 0;
    hori = 0;
    let mut aim = 0;

    for command in &commands {
        match command.dir {
            Dir::Forward => {
                hori += command.amount;
                vert += aim * command.amount;
            },
            Dir::Down =>    { aim += command.amount },
            Dir::Up =>      { aim -= command.amount },
        }
    }
    println!("Part 2: {}", vert * hori);
}
