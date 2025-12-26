// place in tuple
// zip, pairs ,sort, unzip and compare the distances

pub fn part1(input: &str) -> i32 {
    let tuples: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let line_nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect();
            (line_nums[0], line_nums[1])
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

//how many times does num appear in right list
pub fn part2(input: &str) -> i32 {
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

    //Raw approach. loop over both arrays and compar O N^2
    // then stop shortly if it's bigger or smaller then
    let mut result = 0;
    for l_el in left.iter() {
        for r_el in right.iter() {
            if l_el == r_el {
                result += l_el
            }
        }
    }
    result
}
