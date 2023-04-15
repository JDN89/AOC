use std::ops::Index;

use nom::complete::bool;

#[derive(Debug, PartialEq)]
pub enum MyError {
    InvalidDigit(char),
}

pub fn to_num(c: char) -> Result<u32, MyError> {
    match c.to_digit(10)

    {
        Some(value) => Ok(value),
        None => Err(MyError::InvalidDigit(c)),
    }
}


pub fn create_grid(input: &str) -> Result<Vec<Vec<u32>>, MyError> {
    let mut grid = vec![];

    for line in input.lines() {
        let row = line
            .chars()
            .map(to_num)
            .collect::<Result<Vec<u32>, MyError>>()?;
        grid.push(row);
    }

    grid_iterator(&grid);
    Ok(grid)
}

pub fn grid_iterator(grid: &Vec<Vec<u32>>) {
    //create a Vec<Vec<boolean>> where alle the values are set to true
    //later we loop over the vec from left to right and right to left
    //and top to bottom and bottom to top and we set the value to false
    // if we encounter a tree that;s higher then the current tree

    let mut visible_trees = vec![vec![false; grid[0].len()]; grid.len()];

    // iteration for x axis
    for col in 0..grid.len() {
        let mut previous_tree_height = 0;
        for row in 0..grid[0].len() {
           // println!("position: row:{}, col:{} current tree height: {}", row, col, current_tree_height);
            if row == 0 {
                println!("new row");
                previous_tree_height = grid[col][row];
                visible_trees[col][row] = true;
            }
            else if grid[col][row] > previous_tree_height {
                visible_trees[col][row] = true;
                println!("col: {}, row: {}, previous tree height: {}", col, row, previous_tree_height);
                previous_tree_height = grid[col][row];
            }
        }
    }

    println!("visible trees: {:?}", visible_trees);
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
}