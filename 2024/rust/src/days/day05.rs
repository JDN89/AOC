// place in tuple
// zip, pairs ,sort, unzip and compare the distances

use std::collections::{HashMap, HashSet};

fn update_is_correct(updates_map: &HashMap<i32, HashSet<i32>>, sub_arr: &[i32], curr: i32) -> bool {
    for &val in sub_arr {
        match updates_map.get(&val) {
            Some(set) => {
                if set.contains(&curr) {
                    return false;
                }
            }
            None => {
                // The fifth update, 61,13,29, is also not in the correct order, since it breaks the rule 29|13.
                // 13 is not a key in my map, but can be present as a value in my map. only when it is not a key but it is a value should we return false
                dbg!(val);
            }
        }
    }
    true
}

pub fn part1(input: &str) -> i32 {
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut mid_indexes: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.contains('|') {
            let instructions: Vec<i32> = line
                .split('|')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            if !instructions.is_empty() {
                let key = instructions[0];
                let value = instructions[1];
                map.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }
        if line.contains(',') {
            let updates: Vec<i32> = line
                .split(',')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            let mut updates_are_correct = true;
            for (i, _update) in updates.iter().enumerate() {
                let curr = updates[i];
                let compare_with_values = &updates[(i + 1)..];
                match update_is_correct(&map, compare_with_values, curr) {
                    true => continue,
                    false => {
                        updates_are_correct = false;
                        break;
                    }
                }
            }
            if updates_are_correct {
                let mid = updates.len() / 2;
                mid_indexes.push(updates[mid]);
            }

            // use hashset
            // create counter set to 0
            // loop over value of udpates and copmaer with curr_value[counter]
            // see if in some of the folowing the cur value is present as a value in the key!, if so return the handle, ander blijf lopen en zet order_correct op true
        }
    }
    dbg!(&mid_indexes);
    mid_indexes.iter().sum()
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
