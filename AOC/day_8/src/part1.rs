#[derive(Debug, PartialEq)]
pub enum MyError {
    InvalidDigit(char),
}

pub fn to_num(c: char) -> Result<u32, MyError> {
    c.to_digit(10).ok_or(MyError::InvalidDigit(c))
}


pub fn create_grid(input: &str) -> Result<u32, MyError> {
    let grid: Result<Vec<Vec<u32>>, _> = input
        .lines()
        .map(|line| line.chars().map(to_num).collect())
        .collect();

    // The line let grid = grid?; is using the question mark operator (?) to perform error propagation.
    // It checks if the grid variable contains a Result variant with an error (i.e., Err).
    // If it does, the function immediately returns with that error.
    // If the grid variable contains an Ok variant, the line extracts the value inside the Ok variant and rebinds it to the grid variable.
    // In the context of the code you provided, the grid variable is a Result<Vec<Vec<u32>>, MyError>.
    // If the Result is an Err variant, the create_grid function returns early with the Err.
    // If it is an Ok variant, the grid variable now contains the value Vec<Vec<u32>>, which can be used later in the function.
    let grid = grid?;
    let visible_trees = grid_iterator(&grid);
    let result = visible_trees.iter().flatten().filter(|&&x| x).count() as u32;
    Ok(result)
}

pub fn grid_iterator(grid: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    //create a Vec<Vec<boolean>> where alle the values are set to true
    //later we loop over the vec from left to right and right to left
    //and top to bottom and bottom to top and we set the value to false
    // if we encounter a tree that;s higher then the current tree

    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    visible_trees = iterate_x_axis_left_to_right(&grid, &mut visible_trees);
    visible_trees = iterate_x_axis_right_to_left(&grid, &mut visible_trees);
    visible_trees = iterate_y_axis_top_to_bottom(&grid, &mut visible_trees);
    visible_trees = iterate_y_axis_bottom_to_top(&grid, &mut visible_trees);
    // println!("visible trees: {:?}", visible_trees);
    visible_trees
}


pub fn iterate_x_axis_left_to_right(grid: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for col in 0..grid.len() {
        let mut previous_tree_height = 0;
        for row in 0..grid[0].len() {
            if row == 0 {
                previous_tree_height = grid[col][row];
                visible_trees[col][row] = true;
            } else if grid[col][row] > previous_tree_height {
                visible_trees[col][row] = true;
                // println!(
                //     "col: {}, row: {}, previous tree height: {}",
                //     col, row, previous_tree_height
                // );
                previous_tree_height = grid[col][row];
            }
        }
    }
    visible_trees.to_vec()
}

pub fn iterate_x_axis_right_to_left(grid: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for col in (0..grid.len()).rev() {
        let mut previous_tree_height = 0;
        for row in (0..grid[0].len()).rev() {
            // we're counting in reverse order
            if row == grid[0].len() - 1 {
                // println!("new row");
                previous_tree_height = grid[col][row];
                visible_trees[col][row] = true;
            } else if grid[col][row] > previous_tree_height {
                visible_trees[col][row] = true;
                // println!(
                //     "col: {}, row: {}, previous tree height: {}",
                //     col, row, previous_tree_height
                // );
                previous_tree_height = grid[col][row];
            }
        }
    }
    visible_trees.to_vec()
}

pub fn iterate_y_axis_top_to_bottom(grid: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row in 0..grid[0].len() {
        let mut previous_tree_height = 0;
        for col in 0..grid.len() {
            // println!("col: {}, row: {},previous tree height {},current tree height {}", col, row, previous_tree_height, grid[col][row]);
            // we're counting in reverse order
            if col == 0 {
                // println!("new col");
                previous_tree_height = grid[col][row];
                visible_trees[col][row] = true;
            } else if grid[col][row] > previous_tree_height {
                visible_trees[col][row] = true;
                // println!(
                //     "col: {}, row: {}, previous tree height: {}",
                //     col, row, previous_tree_height
                // );
                previous_tree_height = grid[col][row];
            }
        }
    }
    visible_trees.to_vec()
}

pub fn iterate_y_axis_bottom_to_top(grid: &Vec<Vec<u32>>, visible_trees: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row in (0..grid[0].len()).rev() {
        let mut previous_tree_height = 0;
        for col in (0..grid.len()).rev() {
            // println!("col: {}, row: {},previous tree height {},current tree height {}", col, row, previous_tree_height, grid[col][row]);
            // we're counting in reverse order
            if col == grid.len() - 1 {
                // println!("new col: {}", col);
                previous_tree_height = grid[col][row];
                visible_trees[col][row] = true;
            } else if grid[col][row] > previous_tree_height {
                visible_trees[col][row] = true;
                // println!(
                //     "col: {}, row: {}, previous tree height: {}",
                //     col, row, previous_tree_height
                // );
                previous_tree_height = grid[col][row];
            }
        }
    }
    visible_trees.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_char_to_num() {
        let input = '4';
        let result = to_num(input);
        assert_eq!(result, Ok(4))
    }

    #[test]
    fn test_iterate_x_axis_left_to_right() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 3],
            vec![7, 8, 9],
        ];

        let mut visible_trees = vec![vec![false; input[0].len()]; input.len()];

        let result = iterate_x_axis_left_to_right(&input, &mut visible_trees);
        let expected = vec![vec![true, true, true], vec![true, true, false], vec![true, true, true]];
        assert_eq!(
            result, expected
        )
    }

    #[test]
    fn test_iterate_y_axis_top_to_bottom() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 3, 9],
        ];

        let mut visible_trees = vec![vec![false; input[0].len()]; input.len()];

        let result = iterate_y_axis_top_to_bottom(&input, &mut visible_trees);
        // check dat er op de juist plekken true staat
        let expected = vec![vec![true, true, true], vec![true, true, true], vec![true, false, true]];
        assert_eq!(
            result, expected
        )
    }

    #[test]
    //attention, we iterate bottom to top left to right, but the vec visbile trees is still filled in from left to right, top to bottom
    fn test_iterate_over_y_axis_bottom_to_top() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 3, 9],
        ];

        let mut visible_trees = vec![vec![false; input[0].len()]; input.len()];

        let result = iterate_y_axis_bottom_to_top(&input, &mut visible_trees);
        //
        let expected = vec![vec![false, false, false], vec![false, true, false], vec![true, true, true]];
        assert_eq!(
            result, expected
        )
    }
}