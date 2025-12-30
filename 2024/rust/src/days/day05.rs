// place in tuple
// zip, pairs ,sort, unzip and compare the distances

use std::collections::{HashMap, HashSet};

fn order_udpates(
    ordering_rules: &HashMap<i32, HashSet<i32>>,
    unordered_page_updates: &mut Vec<i32>,
) {
    // sort by sorts in ascending order
    //So if or ordering rule key (i) has as value (i+1) we have to make sure that i is in the beginnen of the array
    // so ordering less, as we order in ascending order
    unordered_page_updates.sort_by(|&a, &b| {
        if ordering_rules.get(&a).map_or(false, |set| set.contains(&b)) {
            std::cmp::Ordering::Less
        } else if ordering_rules.get(&b).map_or(false, |set| set.contains(&a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

fn page_ordering_is_correct(
    updates_map: &HashMap<i32, HashSet<i32>>,
    sub_arr: &[i32],
    curr: i32,
) -> bool {
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
            let page_ordering_rules: Vec<i32> = line
                .split('|')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            if !page_ordering_rules.is_empty() {
                let key = page_ordering_rules[0];
                let value = page_ordering_rules[1];
                map.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }
        if line.contains(',') {
            let page_updates: Vec<i32> = line
                .split(',')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            let mut updates_are_correct = true;
            for (i, _update) in page_updates.iter().enumerate() {
                let curr = page_updates[i];
                let compare_with_values = &page_updates[(i + 1)..];
                match page_ordering_is_correct(&map, compare_with_values, curr) {
                    true => continue,
                    false => {
                        updates_are_correct = false;
                        break;
                    }
                }
            }
            if updates_are_correct {
                let mid = page_updates.len() / 2;
                mid_indexes.push(page_updates[mid]);
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
    let mut incorrect_page_updates: Vec<Vec<i32>> = Vec::new();
    let mut page_update_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut mid_indexes: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.contains('|') {
            let page_ordering_rules: Vec<i32> = line
                .split('|')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            if !page_ordering_rules.is_empty() {
                let key = page_ordering_rules[0];
                let value = page_ordering_rules[1];
                page_update_rules
                    .entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(value);
            }
        }
        if line.contains(',') {
            let page_updates: Vec<i32> = line
                .split(',')
                .filter_map(|l| l.parse::<i32>().ok())
                .collect();
            for (i, _update) in page_updates.iter().enumerate() {
                let curr = page_updates[i];
                let compare_with_values = &page_updates[(i + 1)..];
                match page_ordering_is_correct(&page_update_rules, compare_with_values, curr) {
                    true => continue,
                    false => {
                        dbg!(&page_updates);
                        // order the incorrect page update
                        // order_udpates(&page_update_rules, &mut page_updates);
                        incorrect_page_updates.push(page_updates.clone());

                        // incorrect_page_updates.push(page_updates.clone()); // alternative was moving page updates out of the
                        break;
                    }
                }
            }

            // use hashset
            // create counter set to 0
            // loop over value of udpates and copmaer with curr_value[counter]
            // see if in some of the folowing the cur value is present as a value in the key!, if so return the handle, ander blijf lopen en zet order_correct op true
        }
    }
    for page in &mut incorrect_page_updates {
        order_udpates(&page_update_rules, page);
        let mid = page.len() / 2;
        mid_indexes.push(page[mid]);
    }
    dbg!(&mid_indexes);
    mid_indexes.iter().sum()
}
