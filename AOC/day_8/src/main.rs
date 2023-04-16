mod part1;
mod part_2;

fn main() {
    let input = read_input().unwrap();
    // match part1::create_grid(&input) {
    //     Ok(value) => println!("{:?}", value),
    //     Err(e) => println!("{:?}", e)
    // }
    match part_2::create_grid(&input) {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("{:?}", e)
    }
}

fn read_input() -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string("src/test.txt")?)
    // Ok(std::fs::read_to_string("src/part1.txt")?)
}
