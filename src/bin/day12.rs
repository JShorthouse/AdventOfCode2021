use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use ahash::{
    AHashMap,
};

#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Large(String),
    Small(String),
}

#[derive(Debug)]
struct Cave {
    edges: Vec<CaveType>,
}

fn to_cavetype(s: &str) -> CaveType {
    if s == "start" {
        CaveType::Start
    } else if s == "end" {
        CaveType::End
    } else if s.chars().nth(0).unwrap().is_lowercase() {
        CaveType::Small(s.to_owned())
    } else {
        CaveType::Large(s.to_owned())
    }
}

fn build_graph(lines: Vec<String>) -> AHashMap<String, Cave> {
    let mut caves = AHashMap::<String, Cave>::new();

    for line in lines {
        let split: Vec<&str> = line.split("-").collect();
        let first_name = split[0];
        let second_name = split[1];
        let first_type = to_cavetype(first_name);
        let second_type = to_cavetype(second_name);

        {
            let first  = caves.entry(first_name.to_owned()) .or_insert(Cave{ edges: Vec::new() });
            first.edges.push(second_type);
        }
        {
            let second = caves.entry(second_name.to_owned()).or_insert(Cave{ edges: Vec::new() });
            second.edges.push(first_type);
        }
    }
    return caves;
}

fn count_paths<'a>(caves: &'a AHashMap::<String, Cave>, start: &'a Cave, visited: &mut Vec<&'a str>, visit_twice: bool) -> i32 {
    let mut count = 0;
    for cave in &start.edges {
        let mut next = None;
        let mut this_visit_twice = visit_twice;
        let mut write_visited = false;
        match cave {
            CaveType::Start => continue,
            CaveType::End => count += 1,
            CaveType::Large(name) => {
                next = Some(name);
            },
            CaveType::Small(name) => {
                if !visited.contains(&name.as_str()) {
                    next = Some(name);
                    write_visited = true;
                } else if visit_twice {
                    next = Some(name);
                    this_visit_twice = false;
                }
            }
        }
        if let Some(name) = next {
            let next_cave = &caves[name];
            if write_visited {
                visited.push(name);
            }
            count += count_paths(caves, &next_cave, visited, this_visit_twice);
            if write_visited {
                visited.pop();
            }
        }
    }
    return count;
}

fn main() {
    let file = File::open("input/12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let caves = build_graph(lines);

    for (key, cave) in &caves {
        println!("{}: {:?}", key, cave);
    }

    let start = &caves["start"];
    let p1_ans = count_paths(&caves, start, &mut Vec::new(), false);
    let p2_ans = count_paths(&caves, start, &mut Vec::new(), true);
    println!("Part 1: {}", p1_ans);
    println!("Part 2: {}", p2_ans);
}

#[test]
fn test() {
    let ex_2 = [
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc",
    ].iter().map(|s| s.to_string()).collect();
    let ex_2_graph = build_graph(ex_2);
    let ex_2_start = &ex_2_graph["start"];
    assert_eq!(19,  count_paths(&ex_2_graph, ex_2_start, &mut Vec::new(), false));
    assert_eq!(103, count_paths(&ex_2_graph, ex_2_start, &mut Vec::new(), true ));


    let ex_3 = [
       "fs-end",
       "he-DX",
       "fs-he",
       "start-DX",
       "pj-DX",
       "end-zg",
       "zg-sl",
       "zg-pj",
       "pj-he",
       "RW-he",
       "fs-DX",
       "pj-RW",
       "zg-RW",
       "start-pj",
       "he-WI",
       "zg-he",
       "pj-fs",
       "start-RW",
    ].iter().map(|s| s.to_string()).collect();
    let ex_3_graph = build_graph(ex_3);
    let ex_3_start = &ex_3_graph["start"];
    assert_eq!(226,  count_paths(&ex_3_graph, ex_3_start, &mut Vec::new(), false));
    assert_eq!(3509, count_paths(&ex_3_graph, ex_3_start, &mut Vec::new(), true ));
}
