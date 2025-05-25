use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day01.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("solution part 1 : {}", part1_solution);
    println!("solution part 2 : {}", part2_solution);
}

//parse numbers
//create to vecs
//sort vecs -> so mutable
//iter and compare difference

pub fn part1(input: &str) -> i32 {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if nums.len() >= 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect();

    // Sort the pairs by their components separately
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part2(input: &str) -> i32 {
    struct Tuple(Vec<i32>, Vec<i32>);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let mut result: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if nums.len() >= 2 {
            vec1.push(nums[0]);
            vec2.push(nums[1]);
        }
    }
    let mut my_tuple = Tuple(vec1, vec2);
    my_tuple.0.sort();
    my_tuple.1.sort();
    for i in 0..my_tuple.0.len() {
        for y in 0..my_tuple.1.len() {
            if my_tuple.0[i] == my_tuple.1[y] {
                result += my_tuple.0[i]
            }
        }
    }

    // Solve part 2
    result
}
