// run specific part with: cargo build --bin part-1 or part-2

use itertools::Itertools;

pub fn process_part1(input: &str) -> u32 {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().expect("item"))
                .sum()
        })
        .max()
        .expect("excted a value here");
    result
}

//use the iter tools crate!!
pub fn process_part2(input: &str) -> i32 {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| {
                    item.parse::<i32>()
                        .expect("item couldn't be parsed to an u32")
                })
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("{}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define a constant input for ease of use
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn day1_part1() {
        assert_eq!(process_part1(INPUT), 24000);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(process_part2(INPUT), 45000);
    }
}

