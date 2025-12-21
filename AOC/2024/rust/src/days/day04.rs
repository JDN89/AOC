pub fn part1(input: &str) -> i32 {
    let xmas: Vec<char> = "XMAS".chars().collect();
    let directions: Vec<(i8, i8)> = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let xmas_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut num_times_xmas_found = 0;

    for (y, row) in xmas_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &xmas[0] {
                for &(dx, dy) in &directions {
                    let mut xmas_index = 1;
                    while xmas_index < xmas.len() {
                        let dir_x = dx * xmas_index as i8;
                        let dir_y = dy * xmas_index as i8;

                        let nx = x as i32 + dir_x as i32;
                        let ny = y as i32 + dir_y as i32;

                        // bounds check
                        if nx < 0
                            || ny < 0
                            || ny as usize >= xmas_grid.len()
                            || nx as usize >= xmas_grid[ny as usize].len()
                        {
                            break;
                        }

                        // ? why no borrow here?
                        if xmas_grid[ny as usize][nx as usize] != xmas[xmas_index] {
                            break;
                        }
                        xmas_index += 1;
                    }
                    if xmas_index == xmas.len() {
                        num_times_xmas_found += 1;
                    }
                }
            }
        }
    }

    num_times_xmas_found
}

// gebaseerd op: https://todd.ginsberg.com/post/advent-of-code/2024/day4/
pub fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();

    // Corners in predictable order: NW, NE, SE, SW
    let corners = [(-1isize, -1isize), (-1, 1), (1, 1), (1, -1)];

    // Valid X-MAS strings
    let valid_patterns = ["MMSS", "MSSM", "SSMM", "SMMS"];

    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != 'A' {
                continue;
            }

            // Collect corner letters safely
            let mut s = String::with_capacity(4);
            for &(dy, dx) in &corners {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny < 0 || ny >= h as isize || nx < 0 || nx >= w as isize {
                    s.push(' '); // placeholder for out-of-bounds
                } else {
                    s.push(grid[ny as usize][nx as usize]);
                }
            }

            if valid_patterns.contains(&s.as_str()) {
                count += 1;
            }
        }
    }

    count
}
