use std::collections::HashMap;

use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day05.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    println!("\nsolution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> i32 {
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in input.lines() {
        if let Some((left, right)) = line.split_once('|') {
            if let (Ok(k), Ok(v)) = (left.trim().parse::<u32>(), right.trim().parse::<u32>()) {
                ordering_rules.entry(k).or_default().push(v);
                // println!("inserted {} -> {}", k, v);
            }
        } else if line.contains(',') {
            let page_updates: Vec<u32> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            for update in &page_updates {
                match ordering_rules.get(update) {
                    Some(values) => println!("found key {} with values {:?}", update, values),
                    None => println!("no entry for key {}", update),
                }
            }
        }
    }

    // Debug print of the final HashMap
    // for (k, v) in &ordering_rules {
    //     println!("{} -> {:?}", k, v);
    // }
    1
}

pub fn part2(input: &str) -> i32 {
    2
}
