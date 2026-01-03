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
    #[inline]
        fn peek_move_with_extra_obstacle(
            &self,
            grid: &Grid<char>,
            extra_obstacle: (i32, i32),
        ) -> NextMove {
            let (nx, ny) = self.next_position();

            // grid edge
            if ny < 0
                || nx < 0
                || ny >= grid.cells.len() as i32
                || nx >= grid.cells[0].len() as i32
            {
                return NextMove::GridEdge;
            }

            // virtual obstacle check FIRST
            if (nx, ny) == extra_obstacle {
                return NextMove::Obstacle;
            }

            // real grid obstacle
            match grid.cells[ny as usize][nx as usize] {
                '#' => NextMove::Obstacle,
                '.' => NextMove::Default,
                _ => NextMove::Illegal,
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
    // TODO: use to index into the new 3 vec [y][x][d]
    #[inline]
    fn idx(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

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

}


fn causes_loop(
    obstacle: (i32, i32),
    grid: &Grid<char>,
    start_pos: (i32, i32),
    start_dir: Direction,
) -> bool {
    let height = grid.cells.len();
    let width = grid.cells[0].len();

    let mut visited = vec![vec![[false; 4]; width]; height];

    let mut guard = Guard::new(start_dir, start_pos);

    // mark initial state
    visited[start_pos.1 as usize][start_pos.0 as usize][start_dir.idx()] = true;

    loop {
        match guard.peek_move_with_extra_obstacle(grid, obstacle) {
            NextMove::GridEdge => return false,

            NextMove::Obstacle => {
                guard.direction = guard.direction.turn();
            }

            NextMove::Default => {
                guard.advance();
            }

            NextMove::Illegal => unreachable!(),
        }

        let x = guard.position.0 as usize;
        let y = guard.position.1 as usize;
        let d = guard.direction.idx();

        if visited[y][x][d] {
            return true; // loop detected
        }

        visited[y][x][d] = true;
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


fn mark_visited(
    (x, y): (i32, i32),
    visited: &mut Vec<Vec<bool>>,
    positions: &mut Vec<(i32, i32)>,
) {
    let (x, y) = (x as usize, y as usize);
    if !visited[y][x] {
        visited[y][x] = true;
        positions.push((x as i32, y as i32));
    }
}


pub fn part2(input: &str) -> i32 {

    // let mut unique_visited_positions: HashSet<(i32, i32)> = HashSet::new();
    // let mut visited_states: HashSet<(i32, i32, Direction)> = HashSet::new();

    let mut start_position = None;
    let mut start_direction = None;

    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid: Grid<char> = Grid::new(cells);

    //Set everything to false, if visited set to true;
    // Faster to index, we won't have the HashSet overhead off hash, store, find,...
    // O(1) indexing
    let mut visited_grid: Vec<Vec<bool>> = vec![vec![false;grid.width];grid.height];
    let mut unique_visited_positions: Vec<(i32,i32)> = Vec::new();

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

    mark_visited(guard.position, &mut visited_grid, &mut unique_visited_positions);

    // First run: collect all unique visited positions
    loop {
        match guard.peek_move(&grid) {
            NextMove::GridEdge => {
                guard.advance();

                mark_visited(guard.position, &mut visited_grid, &mut unique_visited_positions);

                break;
            }
            NextMove::Obstacle => {
                guard.direction = guard.direction.turn();
            }
            NextMove::Default => {
                guard.advance();

                mark_visited(guard.position, &mut visited_grid, &mut unique_visited_positions);


            }
            NextMove::Illegal => panic!("Illegal move at {:?}", guard.position),
        }
    }


    let loop_counter: usize = unique_visited_positions        .iter()
        .map(|&pos| {
            if causes_loop(pos, &grid, start_position.unwrap(), start_direction.unwrap()) {
                1
            } else {
                0
            }
        })
        .sum();

    loop_counter as i32

}
