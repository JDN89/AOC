// place in tuple
// zip, pairs ,sort, unzip and compare the distances

use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in input.lines() {
        if line.contains('|') {
            let instructions: Vec<i32> = line
                .split('|')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            if !instructions.is_empty() {
                let key = instructions[0];
                let value = instructions[1];
                if map.contains_key(&key) {
                    map.entry(key).or_insert_with(Vec::new).push(value);
                } else {
                    map.insert(key, Vec::new());
                }
            }
        }
        if line.contains(',') {
            let mut set: Vec<i32> = line
                .split(',')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
// use hashset
// create counter set to 0
// loop over value of udpates and copmaer with curr_value[counter]
// see if in some of the folowing the cur value is present as a value in the key!, if so return the handle, ander blijf lopen en zet order_correct op true
            set.reverse();
            for key in set {
                if
            }
        }
    }
    dbg!("| in {}", map.values());
    0
}

//how many times does num appear in right list
pub fn part2(input: &str) -> i32 {
    let tuples: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let line_nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            if line_nums.len() >= 2 {
                // wrap in Some, maar de Some wordt er door de filter_map van uit gefiltered
                return Some((line_nums[0], line_nums[1]));
            } else {
                None
            }
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
    left.sort();
    right.sort();

    //Raw approach. loop over both arrays and compar O N^2
    // then stop shortly if it's bigger or smaller then
    let mut result = 0;
    for l_el in left.iter() {
        for r_el in right.iter() {
            if l_el == r_el {
                result += l_el
            }
        }
    }
    result
}
