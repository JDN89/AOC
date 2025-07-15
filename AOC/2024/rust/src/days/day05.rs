use std::collections::HashMap;

use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day05.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    println!("\nsolution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> u32 {
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut counter = 0;
    for line in input.lines() {
        let mut is_order_correct = true;
        if let Some((left, right)) = line.split_once('|') {
            if let (Ok(k), Ok(v)) = (left.trim().parse::<u32>(), right.trim().parse::<u32>()) {
                ordering_rules.entry(k).or_default().push(v);
                // println!("inserted {} -> {}", k, v);
            }
        } else if line.contains(',') {
            let page_updates: Vec<u32> = line
                .split(',')
                .filter_map(|n| n.parse().ok())
                .rev()
                .collect();
            for window in page_updates.windows(2) {
                let curr_update = window[0];
                let next_update = window[1];
                match &ordering_rules.get(&curr_update) {
                    Some(values) => {
                        if values.contains(&next_update) {
                            println!(
                                "order is wrong for curr {} next {}",
                                curr_update, next_update
                            );
                            is_order_correct = false;
                            break;
                        } else {
                            continue;
                        }
                    }
                    None => println!("ket {} doesn't have a value", curr_update),
                }
            }
            if is_order_correct {
                let mid = page_updates.len() / 2;
                let value = *page_updates.get(mid).unwrap_or(&0);
                counter += value;
                println!("line is correct {:?}", line);
            }
        }
    }

    // Debug print of the final HashMap
    // for (k, v) in &ordering_rules {
    //     println!("{} -> {:?}", k, v);
    // }
    counter
}

pub fn part2(input: &str) -> i32 {
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut counter = 0;
    for line in input.lines() {
        let mut is_order_correct = true;
        if let Some((left, right)) = line.split_once('|') {
            if let (Ok(k), Ok(v)) = (left.trim().parse::<u32>(), right.trim().parse::<u32>()) {
                ordering_rules.entry(k).or_default().push(v);
                // println!("inserted {} -> {}", k, v);
            }
        } else if line.contains(',') {
            let page_updates: Vec<u32> = line
                .split(',')
                .filter_map(|n| n.parse().ok())
                .rev()
                .collect();
            for window in page_updates.windows(2) {
                let curr_update = window[0];
                let next_update = window[1];
                match &ordering_rules.get(&curr_update) {
                    Some(values) => {
                        if values.contains(&next_update) {
                            println!(
                                "order is wrong for curr {} next {}",
                                curr_update, next_update
                            );
                            is_order_correct = false;
                            break;
                        } else {
                            continue;
                        }
                    }
                    None => println!("ket {} doesn't have a value", curr_update),
                }
            }
            if is_order_correct {
                let mid = page_updates.len() / 2;
                let value = *page_updates.get(mid).unwrap_or(&0);
                counter += value;
                println!("line is correct {:?}", line);
            }
        }
    }

    // Debug print of the final HashMap
    // for (k, v) in &ordering_rules {
    //     println!("{} -> {:?}", k, v);
    // }
    counter;
    2
}
