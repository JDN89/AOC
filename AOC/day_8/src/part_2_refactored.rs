#[derive(Debug, PartialEq)]
pub enum MyError {
    InvalidDigit(char),
}

#[derive(Debug, PartialEq)]
pub enum ScenicScoreError {
    InvalidInput(String),
}

pub fn to_num(c: char) -> Result<u32, MyError> {
    c.to_digit(10).ok_or(MyError::InvalidDigit(c))
}

pub fn create_grid(input: &str) -> Result<u32, ScenicScoreError> {
    let grid: Result<Vec<Vec<u32>>, _> = input
        .lines()
        .map(|line| line.chars().map(to_num).collect())
        .collect();

    let grid = grid.map_err(|_| ScenicScoreError::InvalidInput("Invalid input".to_string()))?;
    grid_iterator(&grid)
}

pub fn grid_iterator(grid: &Vec<Vec<u32>>) -> Result<u32, ScenicScoreError> {
    find_highest_scenic_score(&grid)
}

pub fn find_highest_scenic_score(grid: &Vec<Vec<u32>>) -> Result<u32, ScenicScoreError> {
    let mut heighest_scenic_score: u32 = 0;
    let row_max_size = grid[0].len();

    for (col_index, row) in grid.iter().enumerate() {
        for (row_index, tree_house_height) in row.iter().enumerate() {
            let mut score = vec![0; 4];

            // Iterate to the right
            for x in (row_index + 1)..row_max_size {
                let tree_to_the_right = grid[col_index][x];
                if tree_house_height > &tree_to_the_right {
                    score[0] += 1;
                } else {
                    score[0] += 1;
                    break;
                }
            }

            // Iterate to the left
            for x in (0..row_index).rev() {
                let tree_to_the_left = grid[col_index][x];
                if tree_house_height > &tree_to_the_left {
                    score[1] += 1;
                } else {
                    score[1] += 1;
                    break;
                }
            }

            // Iterate upwards
            for y in (0..col_index).rev() {
                let tree_above = grid[y][row_index];
                if tree_house_height > &tree_above {
                    score[2] += 1;
                } else {
                    score[2] += 1;
                    break;
                }
            }

            // Iterate downwards
            for y in (col_index + 1)..grid.len() {
                let tree_above = grid[y][row_index];
                if tree_house_height > &tree_above {
                    score[3] += 1;
                } else {
                    score[3] += 1;
                    break;
                }
            }

            // Calculate scenic score and update the highest scenic score
            let scenic_score: u32 = score.iter().product();
            if scenic_score > heighest_scenic_score {
                heighest_scenic_score = scenic_score;
            }
        }
    }

    Ok(heighest_scenic_score)
}