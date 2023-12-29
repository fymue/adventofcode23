const START_POSITION_SYMBOL: char = 'S';

// a pipe can at most be connected to 2 adjacent pipes
const MAX_ADJACENT_PIPES: usize = 2;
pub const LOOP_CLOCKWISE: usize = 0;
pub const LOOP_COUNTERCLOCKWISE: usize = 1;

enum AdjacentDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GridPosition {
    row: usize,
    col: usize,
}

impl GridPosition {
    pub fn get_adjacent_pipes(&self, maze: &Vec<&str>) -> AdjacentPipes {
        let mut adjacent_pipes: AdjacentPipes =
            [GridPosition{row: 0, col: 0}, GridPosition{row: 0, col: 0}];

        let max_row_idx: usize = maze.len() - 1;
        let max_col_idx: usize = maze[0].len() - 1;

        // TODO: parse the symbol and figure out which two possible adjacent
        //       pipes fit the symbol; then call get_adjacent_pipe for those
        //       and return the updated AdjacentPipes array

        return adjacent_pipes;
    }

    fn get_adjacent_pipe(&self,
        max_row_idx: usize,
        max_col_idx: usize,
        direction: AdjacentDirection) -> GridPosition {
        let adjacent_pipe: GridPosition = match direction {
            AdjacentDirection::NORTH => {
                assert!(self.row - 1 >= 0);
                GridPosition{row: self.row - 1, col: self.col}
            },
            AdjacentDirection::SOUTH => {
                assert!(self.row + 1 <= max_row_idx);
                GridPosition{row: self.row + 1, col: self.col}
            },
            AdjacentDirection::EAST => {
                assert!(self.col + 1 <= max_col_idx);
                GridPosition{row: self.row, col: self.col + 1}
            },
            AdjacentDirection::WEST => {
                assert!(self.col - 1 >= 0);
                GridPosition{row: self.row, col: self.col - 1}
            },
        };

        return adjacent_pipe;
    }
}

type AdjacentPipes = [GridPosition; MAX_ADJACENT_PIPES];
type LoopDirection = usize;

pub fn parse_pipe_maze(file_content: &String) -> Vec<&str> {
    let pipe_maze: Vec<&str> = file_content.trim().split("\n").collect();

    // make sure all lines are the same length
    assert!(pipe_maze.iter().all(|l| l.len() == pipe_maze[0].len()));

    return pipe_maze;
}

pub fn find_start_pipe(pipe_maze: &Vec<&str>) -> Option<GridPosition> {
    for (x, pipe_row) in pipe_maze.iter().enumerate() {
        for (y, grid) in pipe_row.chars().enumerate() {
            if grid == START_POSITION_SYMBOL {
                return Some(GridPosition{row: x, col: y});
            }
        }
    }

    return None;
}

pub fn find_furthest_distance_from_start(
    start_pipe: GridPosition,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> u32 {
    // find the pipe loop starting from start_pipe and collect
    // the grid positions of each pipe in the loop
    let pipe_loop: Vec<GridPosition> =
        find_pipe_loop(start_pipe, pipe_maze, direction);

    // get the total steps it takes to complete a walk through the pipe loop
    let total_steps: u32 = pipe_loop.len() as u32;

    // furthest distance from start is simply half of the total steps
    let mut furthest_distance_from_start: u32 = total_steps / 2;

    // if the step count is uneven, we need to add 1 to maximize it
    if total_steps % 2 != 0 {
        furthest_distance_from_start += 1;
    }

    return furthest_distance_from_start;
}

// just for debugging
#[cfg(debug_assertions)]
fn find_pipe_loop(
    start_pipe: GridPosition,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> Vec<GridPosition> {
    let mut pipe_loop: Vec<GridPosition> = Vec::new();

    // move to the first adjacent pipe of the starting pipe/position
    let mut current_pipe: GridPosition =
        find_next_pipe(&start_pipe, &pipe_maze, &direction);

    let max_loop_iterations: u32 =
        (pipe_maze.len() * pipe_maze[0].len()) as u32;
    let mut loop_iterations: u32 = 0;

    // keep moving to the next adjacent pipe until the loop is
    // completed and we are back at the starting pipe/position
    while current_pipe != start_pipe {
        current_pipe = find_next_pipe(&current_pipe, &pipe_maze, &direction);
        pipe_loop.push(current_pipe);

        loop_iterations += 1;

        // catch infinite loops for debugging purposes (or wrong input)
        if loop_iterations > max_loop_iterations {
            assert!(false, "Pipe loop can't be found!");
        }
    }

    return pipe_loop;
}

#[cfg(not(debug_assertions))]
fn find_pipe_loop(
    start_pipe: GridPosition,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> Vec<GridPosition> {
    let mut pipe_loop: Vec<GridPosition> = Vec::new();

    // move to the first adjacent pipe of the starting pipe/position
    let mut current_pipe: GridPosition =
        find_next_pipe(&start_pipe, &pipe_maze, &direction);

    // keep moving to the next adjacent pipe until the loop is
    // completed and we are back at the starting pipe/position
    while current_pipe != start_pipe {
        current_pipe = find_next_pipe(&current_pipe, &pipe_maze, &direction);
        pipe_loop.push(current_pipe);
    }

    return pipe_loop;
}

#[inline(always)]
fn find_next_pipe(
    current_pipe: &GridPosition,
    pipe_maze: &Vec<&str>,
    direction: &LoopDirection) -> GridPosition {
    return current_pipe.get_adjacent_pipes(pipe_maze)[*direction];
}
