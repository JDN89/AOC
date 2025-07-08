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
        println!(" Horizontal left to right -- x: {}, y: {}", x, y);
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
        println!(" Horizontal right to left -- x: {}, y: {}", x, y);
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
        println!("Vertical bottom to top -- x: {}, y: {}", x, y);
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
        println!("Diagonal top to bottom right to left -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_diagnoal_bottom_right_left(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x < 3 || y < 3 {
        return 0;
    }
    if grid[x - 1][y - 1] == 'M' && grid[x - 2][y - 2] == 'A' && grid[x - 3][y - 3] == 'S' {
        println!("Diagonal bottom to top right to left -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

fn check_diagnoal_bottom_left_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if x < 3 || y + 3 > grid.len() - 1 {
        return 0;
    }
    if grid[x - 1][y + 1] == 'M' && grid[x - 2][y + 2] == 'A' && grid[x - 3][y + 3] == 'S' {
        println!("Diagonal bottom to top left to right -- x: {}, y: {}", x, y);
        return 1;
    }
    return 0;
}

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

pub fn part2(_input: &str) -> i32 {
    1
}
