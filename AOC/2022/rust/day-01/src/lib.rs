pub fn process_part1(input: &str) -> u32 {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item|
                    item.parse::<u32>().expect("couldnt parse"))
                .sum::<u32>()
        })
        .max()
        .expect("expected a value here");
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process_part1("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
        assert_eq!(result, 24000);
    }
}