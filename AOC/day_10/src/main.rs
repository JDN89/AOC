// addx += 2 cycles
// noop += 1 cycle
// 20 th cycle result
// 60th 100th adn 140th cycle result
//
/* use nom::character::complete::char as nom_char; // Use an alias for 'char'

use nom::{
    IResult,
    character::complete::digit1,
    bytes::complete::tag,
    combinator::{opt, map_res},
}; */

/* use nom::sequence::{preceded, tuple}; */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let commands: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    task_2(&commands);
}

fn execute_cycle_actions(cycle_count: &mut i32, x_register: i32, lit_pixels: &mut Vec<i32>) {
    *cycle_count += 1;
    // Check if the cycle count is within the sprite range
    if x_register <= *cycle_count && *cycle_count <= x_register + 2 {
        // This step records the positions of lit pixels (i.e., "#") in the current row.
        lit_pixels.push(*cycle_count - 1);
    }
    //true -> end of row is reached
    if *cycle_count == 40 {
        for index in 0..40 {
            //iterate over the positions 0..39
            //print # if contains index
            if lit_pixels.contains(&index) {
                print!("#");
            } else {
                print!(".");
            }
        }
        // After printing all the positions in the row, print a newline character to move to the next row
        println!();
        lit_pixels.clear();
        *cycle_count = 0;
    }
}

fn task_2(commands: &[String]) {
    let mut lit_pixels = Vec::new();
    let mut x_register = 1;
    let mut cycle_count = 0;

    println!("Task 2 result: ");
    for command in commands {
        execute_cycle_actions(&mut cycle_count, x_register, &mut lit_pixels);
        if command.starts_with("addx") {
            execute_cycle_actions(&mut cycle_count, x_register, &mut lit_pixels);
            x_register += command
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }
}

//part 1:

/* fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut cycle_check = 20;
    let mut cycle_count = 0;
    let mut x_register = 1;
    let mut signal_strength = 0;

    for line in lines {

        cycle_count += 1;
        if line.starts_with("addx") {
            cycle_count +=1;
            if cycle_count >= cycle_check {
                signal_strength += x_register * cycle_check;
                cycle_check += 40;
            }
            let result = parse_instruction(line);
            x_register += result.as_ref().unwrap().1;
        }
    }
    println!("result task 1: {:?}",signal_strength);
}

pub fn parse_instruction(ins: &str) -> IResult<&str, i32> {
    let mut instruction = preceded(tag("addx "), parse_signed_number);
    let (reminder, result) = instruction(ins)?;
    Ok((reminder, result))
}

fn parse_signed_number(input: &str) -> IResult<&str, i32> {
    map_res(
        tuple((opt(nom_char('-')), digit1)),
        |(sign, digits): (Option<char>, &str)| {
            let num = digits.parse::<i32>();
            match num {
                Ok(mut num) => {
                    if sign.is_some() {
                        num = -num;
                    }
                    Ok(num)
                }
                Err(e) => Err(e),
            }
        },
    )(input)
} */
