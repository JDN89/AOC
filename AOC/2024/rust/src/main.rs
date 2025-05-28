mod days;
mod util;

fn main() {
    let day = std::env::args().nth(1).expect("Provide day number");
    match day.as_str() {
        "1" => days::day01::run(),
        "2" => days::day02::run(),
        _ => eprintln!("Unknown day: {}", day),
    }
}
