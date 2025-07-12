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
        (1, 1),   //down right
        (1, -1),  //down left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];
    // let directions: [(isize, isize); 1] = [
    //     // (1, 1),   //down right
    //     // (1, -1),  //down left
    //     (-1, 1), // up-right
    //              // (-1, -1), // up-left
    // ];

    let mut counter: i32 = 0;
    // place the chars of input in vec of vec
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    //len returns number of el in array. zero based index, so len -1
    let row = grid.len() - 1;
    let col = grid[0].len() - 1;

    for x in 0..=row {
        for y in 0..=col {
            if grid[x][y] == 'M' {
                let words = ['A', 'S'];
                // TODO place in seperate funciton that returns a number
                // TODO loop over directions
                let mut words_found = 0;
                let mut directions_found = 0;

                for direction in directions {
                    for i in 0..2 {
                        // NOTE convert to negative
                        let x_isize = x as isize;
                        let y_isize = y as isize;

                        let dx = x_isize + (direction.0 * (i + 1)) as isize;
                        let dy = y_isize + (direction.1 * (i + 1)) as isize;

                        // Boundary check
                        if dx < 0 || dy < 0 || dx > row as isize || dy > col as isize {
                            break; // Out of bounds, stop checking this direction
                        }

                        let dx = dx as usize;
                        let dy = dy as usize;

                        if grid[dx][dy] == words[i as usize] {
                            words_found += 1;
                            if words_found == 2 {
                                directions_found += 1;
                                if directions_found == 2 {
                                    println!("borth directions found for for x: {}, y :{}  ", x, y);
                                }
                                println!("HIT for x: {}, y :{}, dx : {}, dy {}", x, y, dx, dy);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    1
}
