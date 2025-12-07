// place in tuple
// zip, pairs ,sort, unzip and compare the distances

pub fn part1(input: &str) -> i32 {
    let tuples: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let line_nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();

            if line_nums.len() >= 2 {
                // wrap in Some, maar de Some wordt er door de filter_map van uit gefiltered
                return Some((line_nums[0], line_nums[1]));
            } else {
                None
            }
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = tuples.into_iter().unzip();
    left.sort();
    right.sort();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum();
    result
}

pub fn part2(input: &str) -> i32 {
    dbg!(input);
    0
}
