use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day02.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    println!("\nsolution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> i32 {
    let mut safe_reports: i32 = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|l| l.parse().ok())
            .collect();

        //continue to the next row if num.len < 2
        if nums.len() < 2 {
            continue;
        }

        let is_increasing = nums[1] > nums[0];
        let mut is_safe = true;

        for pair in nums.windows(2) {
            let diff = (pair[1] - pair[0]).abs();
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
            if is_increasing && pair[1] <= pair[0] {
                is_safe = false;
                break;
            } else if !is_increasing && pair[1] >= pair[0] {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn is_safe_sequence(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }
    let is_increasing = nums[1] > nums[0];
    for pair in nums.windows(2) {
        let diff = (pair[1] - pair[0]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if is_increasing && pair[1] <= pair[0] {
            return false;
        } else if !is_increasing && pair[1] >= pair[0] {
            return false;
        }
    }
    true
}

// my og mistake was that I removed only first option that failed instead of tying to remove i or
// i+1
// now I just try all the sequences
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|l| l.parse().ok())
                .collect();

            if is_safe_sequence(&nums) {
                return true;
            }

            // Try removing each element once
            for i in 0..nums.len() {
                let mut cloned = nums.clone();
                cloned.remove(i);
                if is_safe_sequence(&cloned) {
                    return true;
                }
            }

            false
        })
        .count() as i32
}
