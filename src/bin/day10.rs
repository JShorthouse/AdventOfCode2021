use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn main() {
    let file = File::open("input/10.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs = Vec::new();
    for line in reader.lines() {
       inputs.push( line.unwrap() );
    }

    let mut completion_stacks: Vec<Vec<char>> = Vec::new();

    let mut illegals = Vec::new();
    for line in inputs {
        let mut valid_line = true;
        let mut stack = Vec::new();
        for ch in line.chars() {
            match ch {
                '{' | '[' | '(' | '<' => {
                    stack.push(ch);
                },
                '}' | ']' | ')' | '>' => {
                    let last = stack.pop();
                    let mut valid = false;
                    if let Some(last) = last {
                        if ch == '}' && last == '{' { valid = true; }
                        else if ch == ']' && last == '[' { valid = true; }
                        else if ch == ')' && last == '(' { valid = true; }
                        else if ch == '>' && last == '<' { valid = true; }
                    }

                    if !valid {
                        illegals.push(ch);
                        valid_line = false;
                        break;
                    }
                },
                _ => panic!("Malformed input"),
            }
        }
        if valid_line {
            completion_stacks.push(stack);
        }
    }

    let mut p1_score = 0;
    for ch in illegals {
        match ch {
            ')' => p1_score += 3,
            ']' => p1_score += 57,
            '}' => p1_score += 1197,
            '>' => p1_score += 25137,
            _ => unreachable!(),
        }
    }

    println!("Part 1: {}", p1_score);

    let mut scores = Vec::<i64>::new();
    for stack in completion_stacks {
        let mut score = 0;
        for ch in stack.iter().rev() {
            score *= 5;
            score += match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
        }
        scores.push(score);
    }
    scores.sort();

    println!("Part 2: {}", scores[scores.len()/2]);
}
