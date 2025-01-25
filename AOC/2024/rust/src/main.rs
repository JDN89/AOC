use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    //
    // arg will be day1 or day2 or test
    let day_or_test = args.get(1).expect("failed to get first arg");

    let input =
        fs::read_to_string(format!("input/{}.txt", day_or_test)).expect("failed to read input");

    match day_or_test.as_str() {
        "test" => println!("input: {}", &input),
        _ => println!("not implemented"),
    }

    println!("Hello, world!");
}
