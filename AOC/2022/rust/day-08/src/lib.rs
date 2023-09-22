fn iterate_left_to_right(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut highest_tree;
    for (y, row) in trees.iter().enumerate() {
        highest_tree = 0;
        for (x, curr_tree) in row.iter().enumerate() {
            if x == 0 || x == row.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_trees[y][x] = true
            }
            if curr_tree > &highest_tree {
                highest_tree = *curr_tree;
                visible_trees[y][x] = true
            }
        }
    }
    visible_trees.to_owned()
}

fn iterate_right_to_left(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut highest_tree;
    for (y, row) in trees.iter().enumerate() {
        highest_tree = 0;
        for x in (0..row.len()).rev() {
            if x == 0 || x == row.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_trees[y][x] = true
            }
            if row[x] > highest_tree {
                highest_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }
    visible_trees.to_owned()
}

fn iterate_top_to_bottom(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut highest_tree;

    for x in 0..trees[0].len() {
        highest_tree = 0;
        for (y, row) in trees.iter().enumerate() {
            if x == 0 || x == row.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_trees[y][x] = true
            }
            if row[x] > highest_tree {
                highest_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }

    visible_trees.to_owned()
}
fn iterate_bottom_to_top(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut previous_tree;

    for x in 0..trees[0].len() {
        previous_tree = 0;
        for (y, row) in trees.iter().enumerate().rev() {
            if x == 0 || x == row.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_trees[y][x] = true
            }
            if row[x] > previous_tree {
                previous_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }

    visible_trees.to_owned()
}

pub fn process_part1(input: &str) -> u32 {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a character"))
                .collect::<Vec<u32>>()
        })
        .collect();
    let len = trees[0].len();
    let mut visible_trees = vec![vec![false; len]; len];
    iterate_left_to_right(&trees, &mut visible_trees);
    iterate_right_to_left(&trees, &mut visible_trees);
    iterate_bottom_to_top(&trees, &mut visible_trees);
    iterate_top_to_bottom(&trees, &mut visible_trees);
    let mut counter: u32 = 0;
    for row in visible_trees.iter() {
        for boolean in row.iter() {
            if boolean == &true {
                counter += 1
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
25532
33549
35390";

    #[test]
    fn test_day8_part1() {
        assert_eq!(process_part1(INPUT), 21);
    }
}
