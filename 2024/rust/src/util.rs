use std::{
    fmt::Display,
    fs::{self},
};

/// Read the entire file into a single String
pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file at path: {}", path))
}

#[derive(Clone)]
pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid.first().map_or(0, |row| row.len());
        Self {
            cells: grid,
            height: height,
            width: width,
        }
    }
    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
    pub fn is_at_egde(&self, x: isize, y: isize) -> bool {
        x == 0 || y == 0 || (x as usize) == (self.width - 1) || (y as usize) == (self.height - 1)
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.is_in_bounds(x, y) {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if self.is_in_bounds(x, y) {
            Some(&mut self.cells[y as usize][x as usize])
        } else {
            None
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
