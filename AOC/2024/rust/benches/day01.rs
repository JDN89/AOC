use aoc_2024::days::day01;
use aoc_2024::util;
use criterion::{criterion_group, criterion_main, Criterion};

// Read the input file once
fn read_input() -> String {
    util::read_input("inputs/input_day01.txt")
}

fn bench_part1(c: &mut Criterion) {
    let input = read_input();
    c.bench_function("day01 part1", |b| {
        b.iter(|| std::hint::black_box(day01::part1(&input)))
    });
}

// fn bench_part2(c: &mut Criterion) {
//     let input = read_input();
//     c.bench_function("day01 part2", |b| {
//         b.iter(|| black_box(day01::part2(&input)))
//     });
// }

criterion_group!(benches, bench_part1);
criterion_main!(benches);
