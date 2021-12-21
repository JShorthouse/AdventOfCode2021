use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use ahash::AHashMap;

fn part_1(p1_starting: i32, p2_starting: i32) -> i32 {
    let mut dice = 1;
    let mut p1_pos = p1_starting;
    let mut p2_pos = p2_starting;
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut p1s_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        let num = dice*3 + 3;
        dice += 3;

        let (mut score, mut pos) = if p1s_turn {
            (p1_score, p1_pos)
        } else {
            (p2_score, p2_pos)
        };

        pos = ((pos + num - 1) % 10) + 1; // add/minus 1 to map 1-10 to 0-9
        score += pos;

        if p1s_turn {
            p1_score = score;
            p1_pos = pos;
        } else {
            p2_score = score;
            p2_pos = pos;
        }

        p1s_turn = !p1s_turn;
    }

    return std::cmp::min(p1_score, p2_score) * (dice-1);
}

// Recursive solve
// Even though numbers are huge worst case max depth will only be 41
// (both players scoring 1 each time until score of 20 and 21), so memory isn't an issue
fn part_2(p1_pos: i32, p2_pos: i32, p1_score: i64, p2_score: i64, p1s_turn: bool,
    cache: &mut AHashMap<(i32, i32, i64, i64, bool), (i64, i64)>) -> (i64, i64)
{
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    if let Some((p1_wins, p2_wins)) = cache.get( &(p1_pos, p2_pos, p1_score, p2_score, p1s_turn) ) {
        return (*p1_wins, *p2_wins);
    }

    for roll in 3..=9 {
        // Number of possible ways to roll each number
        let num_universes = match roll {
            3 => 1,  // 111
            4 => 3,  // 112 121 211
            5 => 6,  // 221 212 122 311 131 133
            6 => 7,  // 321 312 213 231 123 132 222
            7 => 6,  // 331 313 133 223 232 223
            8 => 3,  // 332 323 233
            9 => 1,  // 333
            _ => unreachable!(),
        };

        let mut p1_score = p1_score;
        let mut p2_score = p2_score;
        let mut p1_pos = p1_pos;
        let mut p2_pos = p2_pos;

        let (mut score, mut pos) = if p1s_turn {
            (p1_score, p1_pos)
        } else {
            (p2_score, p2_pos)
        };

        pos = ((pos + roll - 1) % 10) + 1; // add/minus 1 to map 1-10 to 0-9
        score += pos as i64;

        if p1s_turn {
            p1_score = score;
            p1_pos = pos;
        } else {
            p2_score = score;
            p2_pos = pos;
        }

        if p1_score >= 21 {
            p1_wins += num_universes;
        } else if p2_score >= 21 {
            p2_wins += num_universes;
        } else {
            let wins = part_2(p1_pos, p2_pos, p1_score, p2_score, !p1s_turn, cache);
            p1_wins += wins.0 * num_universes;
            p2_wins += wins.1 * num_universes;
        }
    }

    cache.insert( (p1_pos, p2_pos, p1_score, p2_score, p1s_turn), (p1_wins, p2_wins) );

    return (p1_wins, p2_wins);
}

fn main() {
    let file = File::open("input/21.txt").unwrap();
    let mut reader = BufReader::new(file);

    let p1_pos;
    let p2_pos;
    {
        let p1_line = reader.by_ref().lines().next().unwrap().unwrap();
        let p2_line = reader.by_ref().lines().next().unwrap().unwrap();

        p1_pos = p1_line.chars().last().unwrap().to_string().parse::<i32>().unwrap();
        p2_pos = p2_line.chars().last().unwrap().to_string().parse::<i32>().unwrap();
    }

    let p1 = part_1(p1_pos, p2_pos);
    println!("Part 1: {}", p1);

    let mut cache = AHashMap::with_capacity(100_000);
    let wins = part_2(p1_pos, p2_pos, 0, 0, true, &mut cache);
    let p2 = std::cmp::max(wins.0, wins.1);
    println!("Part 2: {:?}", p2);
}
