use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day01.txt");

    let part1_solution = part1(&input);
    let part2_solution = part2(&input);

    println!("solution part 1 : {}", part1_solution);
    println!("solution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> i32 {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            dbg!(line); // show the raw line

            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| {
                    x.parse::<i32>().ok().map(|v| {
                        dbg!(v); // show parsed numbers
                        v
                    })
                })
                .collect();

            dbg!(&nums); // show the numbers vector

            if nums.len() >= 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect();

    dbg!(&pairs); // show all pairs collected

    // Sort the pairs by their components separately
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    vec1.sort();
    vec2.sort();

    dbg!(&vec1, &vec2); // show sorted vectors

    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| {
            let diff = (a - b).abs();
            dbg!(a, b, diff); // show difference
            diff
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        dbg!(line); // show line

        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| {
                x.parse::<i32>().ok().map(|v| {
                    dbg!(v); // show parsed numbers
                    v
                })
            })
            .collect();

        if nums.len() >= 2 {
            vec1.push(nums[0]);
            vec2.push(nums[1]);
        }
    }

    dbg!(&vec1, &vec2); // show vectors before sorting

    vec1.sort();
    vec2.sort();

    dbg!(&vec1, &vec2); // show vectors after sorting

    let mut result = 0;

    for a in &vec1 {
        for b in &vec2 {
            if a == b {
                dbg!(a, b); // show matches
                result += a;
            }
        }
    }

    dbg!(result); // final result
    result
}
