/*  The map shows the current position of the guard with ^ (to indicate the guard is currently facing up from the perspective of the map).

Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.
Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

If there is something directly in front of you, turn right 90 degrees.
Otherwise, take a step forward.
Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):

This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):
*/

use std::collections::HashSet;

use crate::util::Grid;

// see if dir is in this table
const DIRECTION_CHAR: [char; 4] = ['<', '>', '^', 'v'];

const OBSTACLE: char = '#';
const DEFAULT: char = '.';

// For DEBUGGING
fn print_grid_with_guard(grid: &Grid<char>, guard: &Guard) {
    println!("New move \n:");
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

// TODO: check if derive copy clone is necesarry, because only one guard?

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

    // doesn't update just reutrn the next position
    fn next_position(&self) -> (i32, i32) {
        let (dx, dy) = self.direction.delta();
        (self.position.0 + dx, self.position.1 + dy)
    }
    fn advance(&mut self) {
        self.position = self.next_position()
    }

    fn peek_move<'a>(&self, grid: &'a Grid<char>) -> NextMove {
        let (x, y) = self.next_position();
        // if grid.is_at_egde(x as isize, y as isize) {
        //     NextMove::GridEdge
        // } else {
        let val = grid.get(x as isize, y as isize);
        match val {
            Some(val) => match val {
                &DEFAULT => {
                    if grid.is_at_egde(x as isize, y as isize) {
                        NextMove::GridEdge
                    } else {
                        NextMove::Default
                    }
                }
                // Don't do edge check i think because we would keep turning
                &OBSTACLE => NextMove::Obstacle,
                _ => NextMove::Illegal,
            },
            None => NextMove::Illegal,
        }
        // }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

// NOTE: Going up on the grid means going down on the y axis. Top left corner of the grid is (x:0, Y:0)
impl Direction {
    const DELTAS: [(i32, i32); 4] = [
        (1, 0),  //Right
        (0, 1),  // Down,
        (-1, 0), // LEFT,
        (0, -1), // UP,
    ];

    // index in to order with self as usize to get the correct direction
    const ORDER: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    fn delta(self) -> (i32, i32) {
        // to get the number related to the direction, cast self as usize
        Self::DELTAS[self as usize]
    }

    fn turn(self) -> Self {
        Self::ORDER[(self as usize + 1) % 4]
    }

    fn start_direction(dir: &char) -> Self {
        match dir {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Direction not supported"),
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
    println!("{grid}");

    let mut guard: Option<Guard> = None;

    // named outer loop so we can break out of it from the inner loop
    'outer: for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if DIRECTION_CHAR.contains(&cell) {
                let direction = Direction::start_direction(cell);
                guard = Some(Guard::new(direction, (x as i32, y as i32)));

                match grid.get_mut(x as isize, y as isize) {
                    // Overwrite the cell value once we found the starting position of the guard
                    Some(val) => *val = '.',
                    None => {}
                }
                break 'outer;

                // dbg!(direction);
                // dbg!(direction.delta());
            }
        }
    }
    let mut guard = guard.expect("No guard found!");

    // Insert the strting position of the guard
    // unique_visited_positions.insert((guard.position.0, guard.position.1, guard.direction));
    unique_visited_positions.insert((guard.position.0, guard.position.1));

    loop {
        match guard.peek_move(&grid) {
            NextMove::GridEdge => {
                guard.advance();
                unique_visited_positions.insert((guard.position.0, guard.position.1));
                dbg!("edge ");
                print_grid_with_guard(&grid, &guard);
                break;
            }
            // BUG: obstacle does not work. When next move is obstacle the player doesn't recoginze it and moves over it!
            NextMove::Obstacle => {
                guard.direction = guard.direction.turn();

                dbg!("obstacle ");
                print_grid_with_guard(&grid, &guard);
                // Advance is niet nodig, we draaien en doen dan weer een match op next position in onze loop
                // guard.advance();
            }
            NextMove::Default => {
                guard.advance();
                unique_visited_positions.insert((guard.position.0, guard.position.1));
                dbg!("default ");
                print_grid_with_guard(&grid, &guard);
            }
            NextMove::Illegal => panic!("illegal move at guard position {:?}", guard.position),
        }
    }

    let count = unique_visited_positions.len() as i32;
    count
}

pub fn part2(input: &str) -> i32 {
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid: Grid<char> = Grid::new(cells);
    println!("{grid}");

    2
}
