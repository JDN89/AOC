/*
The map shows the guard’s current position using a directional character
(^, v, <, >), indicating the direction the guard is facing.

Obstructions such as crates, desks, or reactors are represented by `#`.

Lab guards in 1518 follow a strict patrol protocol:
- If there is an obstacle directly in front of you, turn right 90 degrees.
- Otherwise, take a step forward.

Following this protocol, the guard continues moving until she eventually
leaves the mapped area.
*/

use std::collections::HashSet;

use crate::util::Grid;

// Characters that indicate the guard's starting direction
const DIRECTION_CHAR: [char; 4] = ['<', '>', '^', 'v'];

const OBSTACLE: char = '#';
const DEFAULT: char = '.';

// Debug helper: prints the grid with the guard rendered at its current position
fn print_grid_with_guard(grid: &Grid<char>, guard: &Guard) {
    println!("New move:\n");
    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if guard.position == (x as i32, y as i32) {
                print!("{}", guard.direction.as_char());
            } else {
                print!("{cell}");
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NextMove {
    GridEdge,
    Obstacle,
    Default,
    Illegal,
}

#[derive(Debug, PartialEq, Eq)]
struct Guard {
    pub direction: Direction,
    pub position: (i32, i32),
}

impl Guard {
    fn new(dir: Direction, pos: (i32, i32)) -> Self {
        Self {
            direction: dir,
            position: pos,
        }
    }

    // Returns the next position without updating the guard state
    fn next_position(&self) -> (i32, i32) {
        let (dx, dy) = self.direction.delta();
        (self.position.0 + dx, self.position.1 + dy)
    }

    fn advance(&mut self) {
        self.position = self.next_position()
    }

    // Inspects the next move without mutating the guard
    fn peek_move(&self, grid: &Grid<char>) -> NextMove {
        let (x, y) = self.next_position();
        let val = grid.get(x as isize, y as isize);

        match val {
            Some(&DEFAULT) => {
                if grid.is_at_egde(x as isize, y as isize) {
                    NextMove::GridEdge
                } else {
                    NextMove::Default
                }
            }
            Some(&OBSTACLE) => NextMove::Obstacle,
            Some(_) => NextMove::Illegal,
            None => NextMove::Illegal,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

// Note: The grid origin (0,0) is the top-left corner.
// Moving "up" means decreasing the y-coordinate.
impl Direction {
    const DELTAS: [(i32, i32); 4] = [
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
        (0, -1), // Up
    ];

    // Clockwise direction order
    const ORDER: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    fn delta(self) -> (i32, i32) {
        Self::DELTAS[self as usize]
    }

    // Turn right (clockwise)
    fn turn(self) -> Self {
        Self::ORDER[(self as usize + 1) % 4]
    }

    fn start_direction(dir: &char) -> Self {
        match dir {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Unsupported direction character"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut unique_visited_positions: HashSet<(i32, i32)> = HashSet::new();

    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid: Grid<char> = Grid::new(cells);

    let mut guard: Option<Guard> = None;

    // Locate the guard's starting position and direction
    'outer: for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if DIRECTION_CHAR.contains(cell) {
                let direction = Direction::start_direction(cell);
                guard = Some(Guard::new(direction, (x as i32, y as i32)));

                // Replace the starting cell with empty space
                if let Some(val) = grid.get_mut(x as isize, y as isize) {
                    *val = DEFAULT;
                }

                break 'outer;
            }
        }
    }

    let mut guard = guard.expect("No guard found");

    // Record the starting position
    unique_visited_positions.insert(guard.position);

    loop {
        match guard.peek_move(&grid) {
            NextMove::GridEdge => {
                guard.advance();
                unique_visited_positions.insert(guard.position);
                break;
            }
            NextMove::Obstacle => {
                guard.direction = guard.direction.turn();
            }
            NextMove::Default => {
                guard.advance();
                unique_visited_positions.insert(guard.position);
            }
            NextMove::Illegal => panic!("Illegal move at {:?}", guard.position),
        }
    }

    unique_visited_positions.len() as i32
}

fn add_obstacle(position: (i32, i32), grid: &mut Grid<char>) {
    let (x, y) = position;
    grid.cells[y as usize][x as usize] = OBSTACLE;
}

pub fn part2(input: &str) -> i32 {
    let mut unique_visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_states: HashSet<(i32, i32, Direction)> = HashSet::new();

    let mut start_position = None;
    let mut start_direction = None;

    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid: Grid<char> = Grid::new(cells);

    let mut guard: Option<Guard> = None;

    // Find the guard's starting state
    'outer: for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if DIRECTION_CHAR.contains(cell) {
                let direction = Direction::start_direction(cell);
                guard = Some(Guard::new(direction, (x as i32, y as i32)));

                start_position = Some((x as i32, y as i32));
                start_direction = Some(direction);

                if let Some(val) = grid.get_mut(x as isize, y as isize) {
                    *val = DEFAULT;
                }

                break 'outer;
            }
        }
    }

    let mut guard = guard.expect("No guard found");
    let base_grid = grid.clone();

    unique_visited_positions.insert(guard.position);

    // First run: collect all visited positions
    loop {
        match guard.peek_move(&grid) {
            NextMove::GridEdge => {
                guard.advance();
                unique_visited_positions.insert(guard.position);
                break;
            }
            NextMove::Obstacle => {
                guard.direction = guard.direction.turn();
            }
            NextMove::Default => {
                guard.advance();
                unique_visited_positions.insert(guard.position);
            }
            NextMove::Illegal => panic!("Illegal move at {:?}", guard.position),
        }
    }

    let mut loop_counter = 0;

    // Try adding an obstacle at each visited position
    for position in unique_visited_positions {
        let mut grid = base_grid.clone();
        visited_states.clear();

        guard.position = start_position.unwrap();
        guard.direction = start_direction.unwrap();

        visited_states.insert((guard.position.0, guard.position.1, guard.direction));
        add_obstacle(position, &mut grid);

        loop {
            match guard.peek_move(&grid) {
                NextMove::GridEdge => {
                    guard.advance();
                    let state = (guard.position.0, guard.position.1, guard.direction);
                    if !visited_states.insert(state) {
                        loop_counter += 1;
                    }
                    break;
                }
                NextMove::Obstacle => {
                    guard.direction = guard.direction.turn();
                    let state = (guard.position.0, guard.position.1, guard.direction);
                    if !visited_states.insert(state) {
                        loop_counter += 1;
                        break;
                    }
                }
                NextMove::Default => {
                    guard.advance();
                    let state = (guard.position.0, guard.position.1, guard.direction);
                    if !visited_states.insert(state) {
                        loop_counter += 1;
                        break;
                    }
                }
                NextMove::Illegal => panic!("Illegal move at {:?}", guard.position),
            }
        }
    }

    loop_counter
}
