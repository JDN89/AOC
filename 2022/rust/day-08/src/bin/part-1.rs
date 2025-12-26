#![feature(test)]

extern crate test;

use day_08::process_part1;
use std::fs;

fn main() {
    let total = fs::read_to_string("input.txt").unwrap();
    let result = process_part1(&total);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::black_box;

    fn input_data() -> String {
        fs::read_to_string("input.txt").unwrap()
    }

    #[test]
    fn part1_works() {
        let total = input_data();
        assert_eq!(1785, process_part1(&total));
    }

    #[bench]
    fn bench_process_part1(b: &mut test::Bencher) {
        let total = input_data();
        b.iter(|| black_box(process_part1(&total)))
    }
}
