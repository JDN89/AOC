use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    match read_username_from_file() {
        Ok() => {}
        Err(_) => {}
    };
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = match y {
        Some(value) => x + value,
        None => x,
    };
    println!("Sum is: {}", sum);
}
