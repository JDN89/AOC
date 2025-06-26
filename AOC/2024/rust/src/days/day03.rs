use crate::util;
use regex::Regex;

pub fn run() {
    let input = util::read_input("inputs/day03.txt");
    let part1_solution = part1(&input);
    // let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    // println!("\nsolution part 2 : {}", part2_solution);
}

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let op1: i32 = caps[1].parse().unwrap();
            let op2: i32 = caps[2].parse().unwrap();
            op1 * op2
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    println!("{}", &input);
    3
}
