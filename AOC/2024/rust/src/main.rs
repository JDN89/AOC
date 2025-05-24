mod days;
mod util;

fn main() {
    let day = std::env::args().nth(1).expect("Povide day number");
    match day.as_str() {
        "inputs/day01.txt" => days::day01::run(),
        // "2" => days::day02::run(),
        _ => eprintln!("Unknown day: {}", day),
    }
    println!("Hello, world!");
}
