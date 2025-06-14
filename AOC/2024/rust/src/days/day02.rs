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
//OLD solution->looks messy
// pub fn part1(input: &str) -> i32 {
//     let mut safe_reports: i32 = 0;
//     let temp: Vel<Vec<i32>> = input
//         .lines()
//         .map(|line| {
//             let nums: Vec<i32> = line
//                 .split_whitespace()
//                 .filter_map(|x| x.parse().ok())
//                 .collect();
//
//             for i in 0..nums.len() - 1 {
//                 println!("i {}, nums[i] {}, nums[i+1] {}", i, nums[i], nums[i + 1]);
//                 if i == nums.len() - 2 {
//                     let diff = (nums[i] - nums[i + 1]).abs();
//                     if diff < 1 || diff > 3 {
//                         break;
//                     } else {
//                         safe_reports += 1;
//                         break;
//                     }
//                 }
//
//                 if nums[i] <= nums[i + 1] {
//                     println!("increasing");
//                     if nums[i + 1] >= nums[i + 2] {
//                         break;
//                     }
//                     let diff = (nums[i] - nums[i + 1]).abs();
//                     if diff < 1 || diff > 3 {
//                         break;
//                     }
//                 } else {
//                     println!("decreasing");
//                     if nums[i + 1] <= nums[i + 2] {
//                         break;
//                     }
//                     let diff = (nums[i] - nums[i + 1]).abs();
//                     if diff < 1 || diff > 3 {
//                         break;
//                     }
//                 }
//             }
//             nums
//         })
//         .collect();
//     safe_reports
// }

pub fn part2(input: &str) -> i32 {
    2
}
