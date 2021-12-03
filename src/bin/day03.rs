use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

fn file_to_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    for line in reader.lines() {
        output.push(line.unwrap())
    }
    return output;
}

fn convert_to_bits<const NUM_BITS: usize>(input: Vec<String>) -> Vec::<[bool; NUM_BITS]> {
    let mut bit_inputs = Vec::<[bool; NUM_BITS]>::new();

    for line in input {
        let mut bits = [false; NUM_BITS];
        for (idx, ch) in line.chars().enumerate() {
            if ch == '1' {
                bits[idx] = true;
            }
        }
        bit_inputs.push(bits);
    }
    return bit_inputs;
}

fn calculate_gamma<const NUM_BITS: usize>(bit_inputs: &Vec<[bool; NUM_BITS]>) -> i32 {
    let mut bit_counts = [0_i32; NUM_BITS];
    for input in bit_inputs {
        for (idx, bit) in input.iter().enumerate() {
            if *bit == true {
                bit_counts[idx] += 1;
            }
        }
    }

    let mut gamma = 0;
    for (idx, count) in bit_counts.iter().rev().enumerate() {
        if *count as usize > (bit_inputs.len()/2) {
            gamma += 1 << idx;
        }
    }
    return gamma;
}

fn g_to_e<const NUM_BITS: usize>(gamma: i32) -> i32 {
    // Generate mask of 1s for number of bits
    let mut mask: i32 = 0;
    for _ in 0..NUM_BITS {
        mask = mask << 1;
        mask += 1;
    }

    let e = !gamma & mask;
    return e;
}

fn p2_search<const NUM_BITS: usize>(inputs: Vec<[bool; NUM_BITS]>, find_most_common: bool) -> i32 {
    let mut found_bits = None;
    let mut remaining = inputs;
    for cur_bit in 0..NUM_BITS {
        let mut bit_count = 0;
        for bits in &remaining {
            if bits[cur_bit] == true {
                bit_count += 1;
            }
        }

        let mut target_bit = false;
        if bit_count >= ((remaining.len() + 1) / 2) { // Round up division
            target_bit = true;
        }
        if !find_most_common {
            target_bit = !target_bit;
        }

        let mut new_remaining = Vec::new();
        for bits in remaining {
            if bits[cur_bit] == target_bit {
                new_remaining.push(bits);
            }
        }

        if new_remaining.len() == 1 {
            found_bits = Some(new_remaining[0]);
            break;
        }

        remaining = new_remaining;
    }

    if let Some(bits) = found_bits {
        // Convert to number
        let mut output = 0;
        for bit in &bits {
            output = output << 1;
            if *bit == true {
                output += 1;
            }
        }
        return output;
    } else {
        unreachable!("Invalid input");
    }
}

fn main() {
    let lines = file_to_lines("input/03.txt");
    let bits = convert_to_bits::<12>(lines);

    let gamma = calculate_gamma(&bits);
    let epsilon = g_to_e::<12>(gamma);
    println!("Part 1: {}", gamma * epsilon);

    let oxygen = p2_search(bits.clone(), true);
    let c02    = p2_search(bits.clone(), false);
    println!("Part 2: {}", oxygen * c02);
}

#[test]
fn test() {
    let input = vec![
        "00100".into(),
        "11110".into(),
        "10110".into(),
        "10111".into(),
        "10101".into(),
        "01111".into(),
        "00111".into(),
        "11100".into(),
        "10000".into(),
        "11001".into(),
        "00010".into(),
        "01010".into(),
    ];
    let bits = convert_to_bits::<5>(input);
    let gamma = calculate_gamma(&bits);
    assert_eq!(gamma, 22);
    assert_eq!(g_to_e::<5>(gamma), 9);

    let oxygen = p2_search(bits.clone(), true);
    let c02    = p2_search(bits.clone(), false);
    assert_eq!(oxygen, 23);
    assert_eq!(c02, 10);
}
