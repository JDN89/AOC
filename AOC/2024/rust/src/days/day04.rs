use std::isize;

use crate::util;

pub fn run() {
    let input = util::read_input("inputs/day04.txt");
    let part1_solution = part1(&input);
    let part2_solution = part2(&input);
    println!("\nsolution part 1 : {}", part1_solution);
    println!("\nsolution part 2 : {}", part2_solution);
}

fn check_horizontal_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    //just temp check if y+3 is bigger then len -> fix later
    if y + 3 > grid[0].len() - 1 {
        return 0;
    }
    if grid[x][y + 1] == 'M' && grid[x][y + 2] == 'A' && grid[x][y + 3] == 'S' {
        // println!(" Horizontal left to right -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_horizontal_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    //just temp check if y+3 is bigger then len -> fix later
    if y < 3 {
        return 0;
    }
    if grid[x][y - 1] == 'M' && grid[x][y - 2] == 'A' && grid[x][y - 3] == 'S' {
        // println!(" Horizontal right to left -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_vertical_top_to_bottom(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    //just temp check if y+3 is bigger then len -> fix later
    if x + 3 > grid.len() - 1 {
        return 0;
    }
    if grid[x + 1][y] == 'M' && grid[x + 2][y] == 'A' && grid[x + 3][y] == 'S' {
        // println!("Vertical top to bottom -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_vertical_bottom_to_top(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    //just temp check if y+3 is bigger then len -> fix later
    if x < 3 {
        return 0;
    }
    if grid[x - 1][y] == 'M' && grid[x - 2][y] == 'A' && grid[x - 3][y] == 'S' {
        // println!("Vertical bottom to top -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}
fn check_diagnoal_top_left_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x + 3 > grid.len() - 1 || y + 3 > grid[0].len() - 1 {
        return 0;
    }
    if grid[x + 1][y + 1] == 'M' && grid[x + 2][y + 2] == 'A' && grid[x + 3][y + 3] == 'S' {
        // println!("Diagonal top to bottom left to right -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}
fn check_diagnoal_top_right_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x + 3 > grid.len() - 1 || y < 3 {
        return 0;
    }
    if grid[x + 1][y - 1] == 'M' && grid[x + 2][y - 2] == 'A' && grid[x + 3][y - 3] == 'S' {
        // println!("Diagonal top to bottom right to left -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_diagnoal_bottom_right_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x < 3 || y < 3 {
        return 0;
    }
    if grid[x - 1][y - 1] == 'M' && grid[x - 2][y - 2] == 'A' && grid[x - 3][y - 3] == 'S' {
        // println!("Diagonal bottom to top right to left -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_diagnoal_bottom_left_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x < 3 || y + 3 > grid.len() - 1 {
        return 0;
    }
    if grid[x - 1][y + 1] == 'M' && grid[x - 2][y + 2] == 'A' && grid[x - 3][y + 3] == 'S' {
        // println!("Diagonal bottom to top left to right -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}
// Elegantere oplossing met behulp van chtgp. je werkt gewoon met directies
// en je doen x en y  * 1,2,3,4 om de coordinaten te bekomen
// let directions = [
//        (0, 1),   // right
//        (0, -1),  // left
//        (1, 0),   // down
//        (-1, 0),  // up
//        (1, 1),   // down-right
//        (1, -1),  // down-left
//        (-1, 1),  // up-right
//        (-1, -1), // up-left
//    ];
pub fn part1(input: &str) -> i32 {
    let mut counter: i32 = 0;
    // place the chars of input in vec of vec
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    //len returns number of el in array. zero based index, so len -1
    let row = grid.len() - 1;
    let col = grid[0].len() - 1;

    //NOTE: itterate over inclusive range ..=
    for x in 0..=row {
        for y in 0..=col {
            if grid[x][y] == 'X' {
                counter += check_horizontal_right(&grid, x, y);
                counter += check_horizontal_left(&grid, x, y);
                counter += check_vertical_top_to_bottom(&grid, x, y);
                counter += check_vertical_bottom_to_top(&grid, x, y);
                counter += check_diagnoal_top_right_left(&grid, x, y);
                counter += check_diagnoal_top_left_right(&grid, x, y);
                counter += check_diagnoal_bottom_left_right(&grid, x, y);
                counter += check_diagnoal_bottom_right_left(&grid, x, y);
            }
        }
        // println!("row{:?}", row);
    }

    return counter;
}

// BUG start over!!
// i have to start checking from A!
// Now i Take M as the starting point wich is wrong
// check only top-right and top-left and see when you encounter A if the other letters are M and S
// then check the other diagonoal.
// if both hit return 1 -> increase counter
// FUN with grids

pub fn part2(input: &str) -> i32 {
    let directions: [(isize, isize); 4] = [
        //Down right diagonal
        (-1, -1), //top-left
        (1, 1),   //down-right
        //Down left diagonal
        (-1, 1), //top-rigth
        (1, -1), //down-left
    ];

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row = grid.len() - 1;
    let col = grid[0].len() - 1;

    let mut xmas_counter = 0;

    for x in 0..=row {
        for y in 0..=col {
            // Look for A and then check for 'S' and 'M' in the diagonal directions
            if grid[x][y] == 'A' {
                // if A is at the edge, break
                if x == 0 || x == grid.len() - 1 || y == 0 || y == grid[0].len() - 1 {
                    continue;
                }
                //cast usize to isize, needed for negative numbers
                let top_left_x = x as isize + directions[0].0;
                let top_left_y = y as isize + directions[0].1;
                let down_right_x = x as isize + directions[1].0;
                let down_right_y = y as isize + directions[1].1;
                let top_right_x = x as isize + directions[2].0;
                let top_right_y = y as isize + directions[2].1;
                let down_left_x = x as isize + directions[3].0;
                let down_left_y = y as isize + directions[3].1;

                if ((grid[top_left_x as usize][top_left_y as usize] == 'M'
                    && grid[down_right_x as usize][down_right_y as usize] == 'S')
                    || (grid[top_left_x as usize][top_left_y as usize] == 'S'
                        && grid[down_right_x as usize][down_right_y as usize] == 'M'))
                    //check if we find M and S on both diagonals
                    && ((grid[top_right_x as usize][top_right_y as usize] == 'M'
                        && grid[down_left_x as usize][down_left_y as usize] == 'S')
                        || (grid[top_right_x as usize][top_right_y as usize] == 'S'
                            && grid[down_left_x as usize][down_left_y as usize] == 'M'))
                {
                    // println!("MAS found! -- x {} y {} ", x + 1, y + 1);
                    xmas_counter += 1;
                }
            }
        }
    }

    xmas_counter
}

//SOLUTION spit out by the AI -> I asked for a cleaner solution
pub fn _part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Helper to safely get a character at (x, y)
    let get = |x: isize, y: isize| {
        if x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols {
            Some(grid[x as usize][y as usize])
        } else {
            None
        }
    };

    for x in 1..(rows - 1) {
        for y in 1..(cols - 1) {
            if grid[x][y] != 'A' {
                continue;
            }

            let x = x as isize;
            let y = y as isize;

            // Diagonal characters
            let top_left = get(x - 1, y - 1);
            let bottom_right = get(x + 1, y + 1);
            let top_right = get(x - 1, y + 1);
            let bottom_left = get(x + 1, y - 1);

            // Check both diagonals contain 'M' and 'S' (any order)
            let diag1 = [top_left, bottom_right];
            let diag2 = [top_right, bottom_left];

            if is_mas_diag(&diag1) && is_mas_diag(&diag2) {
                count += 1;
            }
        }
    }

    count
}

fn is_mas_diag(diag: &[Option<char>; 2]) -> bool {
    match (diag[0], diag[1]) {
        (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
        _ => false,
    }
}
