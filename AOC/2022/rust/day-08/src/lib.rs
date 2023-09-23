fn set_boundries_as_visible(visible_trees: &mut Vec<Vec<bool>>) {
    let rows = visible_trees.len();
    let cols = visible_trees[0].len();

    for x in 0..cols {
        visible_trees[0][x] = true;
        visible_trees[rows - 1][x] = true;
    }
    for y in 0..rows {
        visible_trees[y][0] = true;
        visible_trees[y][cols - 1] = true
    }
}

fn iterate_left_to_right(trees: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) {
    let mut highest_tree;
    for (y, row) in trees.iter().enumerate() {
        highest_tree = 0;
        for (x, curr_tree) in row.iter().enumerate() {
            if curr_tree > &highest_tree {
                highest_tree = *curr_tree;
                visible_trees[y][x] = true
            }
        }
    }
}

fn iterate_right_to_left(trees: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) {
    let mut highest_tree;
    for (y, row) in trees.iter().enumerate() {
        highest_tree = 0;
        for x in (0..row.len()).rev() {
            if row[x] > highest_tree {
                highest_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }
}

fn iterate_top_to_bottom(trees: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) {
    let mut highest_tree;

    for x in 0..trees[0].len() {
        highest_tree = 0;
        for (y, row) in trees.iter().enumerate() {
            if row[x] > highest_tree {
                highest_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }
}
fn iterate_bottom_to_top(trees: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) {
    let mut previous_tree;

    for x in 0..trees[0].len() {
        previous_tree = 0;
        for (y, row) in trees.iter().enumerate().rev() {
            if row[x] > previous_tree {
                previous_tree = row[x];
                visible_trees[y][x] = true
            }
        }
    }
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
    set_boundries_as_visible(&mut visible_trees);
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
