
pub fn process_part1(input: &str) {
    println!("{:?}", input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn it_works() {
        let result = process_part1("test_input.txt");
        assert_eq!(result, 24000);
    }
}