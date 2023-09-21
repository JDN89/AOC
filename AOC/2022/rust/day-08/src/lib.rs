fn iterate_left_to_right(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut previous_tree = 0;
    for (y, row) in trees.iter().enumerate() {
        previous_tree = 0;
        'inner: for (x, curr_tree) in row.iter().enumerate() {
            if curr_tree > &previous_tree {
                previous_tree = *curr_tree
            } else {
                visible_trees[y][x] = false;
                break 'inner;
            }

            // dbg!("rij {} waarde{}, kolom{} waarde {}", x, row, y, col);
        }
    }
    visible_trees.to_owned()
}

fn iterate_right_to_left(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut previous_tree = 0;
    for (y, row) in trees.iter().enumerate() {
        previous_tree = 0;
        'inner: for (x, curr_tree) in row.iter().rev().enumerate() {
            if curr_tree > &previous_tree {
                previous_tree = *curr_tree
            } else {
                visible_trees[y][x] = false;
                break 'inner;
            }
        }
    }
    visible_trees.to_owned()
}

fn iterate_top_to_bottom(
    trees: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
) -> Vec<Vec<bool>> {
    let mut previous_tree = 0;

    for x in 0..trees[0].len() {
        previous_tree = 0;
        'inner: for (y, row) in trees.iter().enumerate() {
            if row[x] > previous_tree {
                previous_tree = row[x]
            } else {
                visible_trees[y][x] = false;
                break 'inner;
            }
        }
    }

    visible_trees.to_owned()
}

pub fn process_part1(input: &str) -> Option<u32> {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a character"))
                .collect::<Vec<u32>>()
        })
        .collect();

    // instantiate a vec of vec of booleans fo the same size and set all the trees to true
    // iteratie left to right and right to left
    // itereate top to bottom and bottom totop
    // set the trees that are ont visible to false
    // loop over grid and count number of trees that is said to true

    let len = trees[0].len();
    let mut visible_trees = vec![vec![true; len]; len];

    Some(3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const INPUT: &str = "30373
25512
25532
33549
35390";

    lazy_static! {
        static ref GRID: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 3], vec![7, 8, 9]];
        static ref INITIAL: Vec<Vec<bool>> = vec![vec![true; 5]; 5];
    }

    #[test]
    fn test_day1_part1() {
        assert_eq!(process_part1(INPUT), Some(21));
    }

    const VISIBLE_TREES_LEFT_TO_RIGHT: &[&[bool]] = &[
        &[true, true, true],
        &[true, false, true],
        &[true, true, true],
    ];

    #[test]
    fn test_iterate_left_to_right() {
        let input = vec![vec![1, 2, 3], vec![6, 5, 3], vec![7, 8, 9]];

        let mut visible_trees = vec![vec![true; input[0].len()]; input.len()];
        assert_eq!(
            iterate_left_to_right(&input, &mut visible_trees),
            VISIBLE_TREES_LEFT_TO_RIGHT
        );
    }

    const VISIBLE_TREES_RIGHT_TO_LEFT: &[&[bool]] = &[
        &[true, false, true],
        &[false, true, true],
        &[true, false, true],
    ];

    #[test]
    fn test_right_to_left() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 3], vec![7, 8, 9]];

        let mut visible_trees = vec![vec![true; input[0].len()]; input.len()];
        assert_eq!(
            iterate_right_to_left(&input, &mut visible_trees),
            VISIBLE_TREES_RIGHT_TO_LEFT
        );
    }

    const VISIBLE_TREES_TOP_TO_BOTTOM: &[&[bool]] = &[
        &[true, true, true],
        &[true, true, false],
        &[true, true, true],
    ];

    #[test]
    fn test_top_to_bottom() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 3], vec![7, 8, 9]];

        let mut visible_trees = vec![vec![true; input[0].len()]; input.len()];
        assert_eq!(
            iterate_top_to_bottom(&input, &mut visible_trees),
            VISIBLE_TREES_TOP_TO_BOTTOM
        );
    }
}
