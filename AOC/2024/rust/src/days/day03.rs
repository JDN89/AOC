use crate::util;
use regex::Regex;

pub fn run() {
    let input = util::read_input("inputs/day03.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    println!("\nsolution part 2 : {}", part2_solution);
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
    let re = Regex::new(
        r"(?P<mul>mul\((?P<mul_x>\d{1,3}),(?P<mul_y>\d{1,3})\))|(?P<do>do\(\))|(?P<don>don't\(\))",
    )
    .unwrap();
    let mut enabled = true;
    let mut total = 0;
    for caps in re.captures_iter(input) {
        if caps.name("do").is_some() {
            enabled = true;
        }
        if caps.name("don").is_some() {
            enabled = false;
        }
        if let Some(_) = caps.name("mul") {
            if enabled {
                //as_str because caps.name returns regex::Match
                let x: i32 = caps.name("mul_x").unwrap().as_str().parse().unwrap();
                let y: i32 = caps.name("mul_y").unwrap().as_str().parse().unwrap();
                total += x * y;
            }
        }
    }
    return total;
}
