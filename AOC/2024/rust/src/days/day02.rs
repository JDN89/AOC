use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day02.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("solution part 1 : {}", part1_solution);
    println!("solution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> i32 {
    let temp: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            nums
        })
        .collect();
    for row in 0..temp.len() {
        for col in 0..temp[0].len() {
            println!("row {} column {}", row + 1, temp[row][col]);
        }
    }
    2
}

pub fn part2(input: &str) -> i32 {
    2
}
