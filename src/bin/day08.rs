use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use std::collections::{
    HashSet,
    HashMap,
};
use std::iter::FromIterator;

struct Signals {
    all: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}


fn main() {
    let file = File::open("input/08.txt").unwrap();
    let reader = BufReader::new(file);

    let mut signal_list = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split = line.split("|").collect::<Vec<&str>>();

        let mut all = Vec::new();
        let mut output = Vec::new();

        for sequence in split[0].split_whitespace() {
            let set = HashSet::from_iter(sequence.chars());
            all.push(set);
        }

        for sequence in split[1].split_whitespace() {
            let set = HashSet::from_iter(sequence.chars());
            output.push(set);
        }

        signal_list.push( Signals{ all, output } );
    }

    let mut p1_count = 0;
    for signals in &signal_list {
        for signal in &signals.output {
            if signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7 {
                p1_count += 1;
            }
        }
    }

    println!("Part 1: {}", p1_count);

    // Num, Segments
    // 1    2  Unique
    // 7    3  Unique
    // 4    4  Unique
    // 2    5  \
    // 3    5   |
    // 5    5  /
    // 6    6  \
    // 9    6   |
    // 0    6  /
    // 8    7  Unique
    //
    // Strategy:
    // Find top: Difference of 1 and 7
    // Find horizontal candiates: Intersection of 235
    //   - Remove top
    //   - Middle is intersection with 4
    //   - Bottom is remaining
    // Find left candiates: only appear once in 235
    // Find right candidates: remaining
    // Find top left: intersection of left candiates and 4
    // Find top right: right candidate that appears twice in 690
    // Bottom left, bottom right: remaining in each

    let mut p2_count = 0;
    for signals in &signal_list {
        //  0000
        // 1    2
        // 1    2
        //  3333
        // 4    5
        // 4    5
        //  6666
        let mut mappings = ['x'; 7];

        let mut remaining = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

        let empty = HashSet::<char>::new();
        let mut one:   &HashSet<char> = &empty;
        let mut four:  &HashSet<char> = &empty;
        let mut seven: &HashSet<char> = &empty;
        let mut two_three_five = Vec::<&HashSet<char>>::new();
        let mut six_nine_zero = Vec::<&HashSet<char>>::new();

        for signal in &signals.all {
            match signal.len() {
                2 => one = &signal,
                3 => seven = &signal,
                4 => four = &signal,
                5 => two_three_five.push(&signal),
                6 => six_nine_zero.push(&signal),
                _ => (),
            }
        }

        { // Top segment
            let top = *seven.difference(one).next().unwrap();
            mappings[0] = top;
            remaining.remove(&top);
        }

        { // Middle and bottom segments
            let rem0    = remaining.clone();
            let rem1    = rem0.intersection(two_three_five[0]).cloned().collect::<HashSet<char>>();
            let rem2    = rem1.intersection(two_three_five[1]).cloned().collect::<HashSet<char>>();
            let mut rem = rem2.intersection(two_three_five[2]).cloned().collect::<HashSet<char>>();

            let middle = rem.intersection(&four).next().unwrap().clone();
            rem.remove(&middle);
            let bottom = rem.iter().next().unwrap().clone();

            mappings[3] = middle;
            mappings[6] = bottom;

            remaining.remove(&middle);
            remaining.remove(&bottom);
        }


        { // Left segments
            let mut counts_235 = HashMap::new();
            for signals in &two_three_five {
                for ch in *signals {
                    let entry = counts_235.entry(ch).or_insert(0);
                    *entry += 1;
                }
            }

            let mut left_segments = Vec::new();
            for (ch, count) in &counts_235 {
                if *count == 1 {
                    left_segments.push(**ch);
                }
            }

            if four.contains(&left_segments[0]) {
                mappings[1] = left_segments[0];
                mappings[4] = left_segments[1];
            } else {
                mappings[1] = left_segments[1];
                mappings[4] = left_segments[0];
            }

            remaining.remove(&left_segments[0]);
            remaining.remove(&left_segments[1]);
        }

        { // Right segments
            let right_segments = remaining.iter().cloned().collect::<Vec<char>>();
            let mut potential_top_right = Vec::new();

            let mut counts_690 = HashMap::new();
            for signals in &six_nine_zero {
                for ch in *signals {
                    let entry = counts_690.entry(ch).or_insert(0);
                    *entry += 1;
                }
            }

            for (ch, count) in &counts_690 {
                if *count == 2 {
                    potential_top_right.push(**ch);
                }
            }

            if potential_top_right.contains(&right_segments[0]) {
                mappings[2] = right_segments[0];
                mappings[5] = right_segments[1];
            } else {
                mappings[2] = right_segments[1];
                mappings[5] = right_segments[0];
            }

            remaining.remove(&right_segments[0]);
            remaining.remove(&right_segments[1]);
        }

        let segment_mappings: [HashSet<char>; 10] = [
            /* 0 */ HashSet::from([mappings[0], mappings[1], mappings[2], mappings[4], mappings[5], mappings[6]]),
            /* 1 */ HashSet::from([mappings[2], mappings[5]]),
            /* 2 */ HashSet::from([mappings[0], mappings[2], mappings[3], mappings[4], mappings[6]]),
            /* 3 */ HashSet::from([mappings[0], mappings[2], mappings[3], mappings[5], mappings[6]]),
            /* 4 */ HashSet::from([mappings[1], mappings[2], mappings[3], mappings[5]]),
            /* 5 */ HashSet::from([mappings[0], mappings[1], mappings[3], mappings[5], mappings[6]]),
            /* 6 */ HashSet::from([mappings[0], mappings[1], mappings[3], mappings[4], mappings[5], mappings[6]]),
            /* 7 */ HashSet::from([mappings[0], mappings[2], mappings[5]]),
            /* 8 */ HashSet::from([mappings[0], mappings[1], mappings[2], mappings[3], mappings[4], mappings[5], mappings[6]]),
            /* 9 */ HashSet::from([mappings[0], mappings[1], mappings[2], mappings[3], mappings[5], mappings[6]]),
        ];

        let mut number = 0;
        for signal in &signals.output {
            number *= 10;
            let mut found = false;
            for (digit, mapping) in segment_mappings.iter().enumerate() {
                if signal == mapping {
                    number += digit;
                    found = true;
                    break;
                }
            }
            if !found{ panic!("Failed to parse {:?}", signal);}
        }

        p2_count += number;
    }

    println!("Part 2: {}", p2_count);
}
