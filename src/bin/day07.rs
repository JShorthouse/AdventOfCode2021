use std::fs::read_to_string;

fn calc_triangle_costs(input: &Vec<i32>, median: i32) -> i32 {
    let mut total_cost = 0;
    for val in input {
        let offset = (median - val).abs();
        total_cost += (offset * (offset + 1))/2;
    }
    total_cost
}

fn run(input: Vec<i32>) -> (i32, i32) {
    let mut input = input;
    input.sort();

    let median = if input.len() % 2 == 0 {
        let left = input[input.len()/2 - 1];
        let right = input[input.len()/2];
        (left + right) / 2
    } else {
        input[input.len()/2]
    };

    let mut p1 = 0;
    for val in &input {
        p1 += (median - val).abs();
    }

    let median_cost = calc_triangle_costs(&input, median);
    let left_cost = calc_triangle_costs(&input, median-1);
    let right_cost = calc_triangle_costs(&input, median+1);

    let direction = if left_cost < median_cost {
        -1
    } else if right_cost < median_cost {
        1
    } else {
        panic!("Couldn't optimize position for part 2");
    };

    let mut optimal_cost = median_cost;
    let mut cur_middle = median;

    // Shift median to find lowest cost
    loop {
        cur_middle += direction;
        let new_cost = calc_triangle_costs(&input, cur_middle);
        if new_cost > optimal_cost {
            break;
        }

        optimal_cost = new_cost;
    }

    (p1, optimal_cost)
}

fn main() {
    let input: Vec<i32> = read_to_string("input/07.txt").unwrap().trim()
                          .split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let (p1, p2) = run(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[test]
fn test() {
    let mut input = vec![16,1,2,0,4,2,7,1,2,14];
    let (p1, p2) = run(input);
    assert_eq!(p1, 37);
    assert_eq!(p2, 168);
}
