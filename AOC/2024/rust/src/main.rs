use std::env;

mod days;
mod util;

fn main() {
    let day = env::args().nth(1).expect("Provide day number");
    let part = env::args().nth(2).expect("Provide part number");
    let input_file = format!("inputs/input_day{}.txt", day);

    let input = util::read_input(&input_file);

    // Map day and part to the corresponding function
    let result = match day.as_str() {
        "01" => match part.as_str() {
            "p1" => Some(days::day01::part1(&input)),
            "p2" => Some(days::day01::part2(&input)),
            _ => None,
        },
        "02" => match part.as_str() {
            "p1" => Some(days::day02::part1(&input)),
            "p2" => Some(days::day02::part2(&input)),
            _ => None,
        },
        "03" => match part.as_str() {
            "p1" => Some(days::day03::part1(&input)),
            "p2" => Some(days::day03::part2(&input)),
            _ => None,
        },
        // Add more days here
        _ => None,
    };

    match result {
        Some(value) => println!("Result: {}", value),
        None => eprintln!("Unknown day ({}) or part ({})", day, part),
    }
}
