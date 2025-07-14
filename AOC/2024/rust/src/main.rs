mod days;
mod util;

fn main() {
    let day = std::env::args().nth(1).expect("Provide day number");
    match day.as_str() {
        "01" => days::day01::run(),
        "02" => days::day02::run(),
        "03" => days::day03::run(),
        "04" => days::day04::run(),
        "05" => days::day05::run(),
        _ => eprintln!("Unknown day: {}", day),
    }
}
