pub const LOOP_CLOCKWISE: usize = 0;
pub const LOOP_COUNTERCLOCKWISE: usize = 1;

const START_POSITION_SYMBOL: char = 'S';

// a pipe can at most be connected to 2 adjacent pipes
const MAX_ADJACENT_PIPES: usize = 2;

pub fn parse_pipe_maze(file_content: &String) -> Vec<&str> {
    let pipe_maze: Vec<&str> = file_content.trim().split("\n").collect();

    // make sure all lines are the same length
    assert!(pipe_maze.iter().all(|l| l.len() == pipe_maze[0].len()));

    return pipe_maze;
}

pub fn find_start_pipe(pipe_maze: &Vec<&str>) -> Option<LoopPipe> {
    for (x, pipe_row) in pipe_maze.iter().enumerate() {
        for (y, grid) in pipe_row.chars().enumerate() {
            if grid == START_POSITION_SYMBOL {
                return Some(LoopPipe{
                    row: x, col: y, pipe_type: PipeType::S});
            }
        }
    }

    return None;
}

pub fn find_furthest_distance_from_start(
    start_pipe: LoopPipe,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> u32 {
    // find the pipe loop starting from start_pipe and collect
    // the grid positions of each pipe in the loop
    let pipe_loop: Vec<LoopPipe> =
        find_pipe_loop(start_pipe, pipe_maze, direction);

    // calculate the furthest distance from the starting position/pipe
    let furthest_distance_from_start: u32 =
        calc_furthest_distance_from_start(pipe_loop);

    return furthest_distance_from_start;
}

// all possible directions in which a pipe can be connected to another pipe
#[derive(Clone, Copy, PartialEq, Debug)]
enum AdjacentDirection {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

// classify pipe types by how they can be connected to other pipes
#[derive(Clone, Copy, PartialEq, Debug)]
enum PipeType {
    S,   // Start-Pipe
    NS,  // '|' (North-South)
    EW,  // '-' (East-West)
    NE,  // 'L' (North-East)
    NW,  // 'J' (North-West)
    SW,  // '7' (South-West)
    SE,  // 'F' (South-East)
}

// representation of a pipe in the single loop of the pipe maze
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LoopPipe {
    row: usize,
    col: usize,
    pipe_type: PipeType,
}

type AdjacentPipes = [LoopPipe; MAX_ADJACENT_PIPES];
type CompatibleDirections = [AdjacentDirection; MAX_ADJACENT_PIPES];
type LoopDirection = usize;

impl LoopPipe {
    pub fn get_adjacent_pipes(&self, maze: &Vec<&str>) -> AdjacentPipes {
        let compatible_directions: CompatibleDirections =
            self.get_compatible_directions(self.pipe_type, maze);
        println!("compatible directions: {:?}, {:?}", compatible_directions[0], compatible_directions[1]);
        
        let adjacent_pipes: AdjacentPipes = [
            self.get_adjacent_pipe(
                compatible_directions[LOOP_CLOCKWISE], maze),
            self.get_adjacent_pipe(
                compatible_directions[LOOP_COUNTERCLOCKWISE], maze)
        ];

        return adjacent_pipes;
    }

    // parse a pipe that is adjacent to the self pipe
    // (assumes that the grid field of the pipe maze in the provided
    //  direction defintely contains a compatible pipe to the self pipe)
    fn get_adjacent_pipe(
        &self,
        direction: AdjacentDirection,
        pipe_maze: &Vec<&str>) -> LoopPipe {
        let max_row_idx: usize = pipe_maze.len() - 1;
        let max_col_idx: usize = pipe_maze[0].len() - 1;
    
        let adjacent_pipe: LoopPipe = match direction {
            AdjacentDirection::NORTH => {
                assert!(self.row > 0);
                assert!(get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::NORTH).is_some());

                let pipe_type: PipeType = get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::NORTH).unwrap();

                LoopPipe{
                    row: self.row - 1, col: self.col, pipe_type: pipe_type}
            },

            AdjacentDirection::SOUTH => {
                assert!(self.row + 1 <= max_row_idx);
                assert!(get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::SOUTH).is_some());

                let pipe_type: PipeType = get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::SOUTH).unwrap();

                LoopPipe{
                    row: self.row + 1, col: self.col, pipe_type: pipe_type}
            },

            AdjacentDirection::EAST => {
                assert!(self.col + 1 <= max_col_idx);
                assert!(get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::EAST).is_some());

                let pipe_type: PipeType = get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::EAST).unwrap();

                LoopPipe{
                    row: self.row, col: self.col + 1, pipe_type: pipe_type}
            },

            AdjacentDirection::WEST => {
                assert!(self.col > 0);
                assert!(get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::WEST).is_some());

                let pipe_type: PipeType = get_pipe_type(
                    self.row, self.col, pipe_maze,
                    AdjacentDirection::WEST).unwrap();

                LoopPipe{
                    row: self.row, col: self.col - 1, pipe_type: pipe_type}
            },
        };

        return adjacent_pipe;
    }

    // figure out the 2 (out of 4 possible) adjacent
    // directions the pipe loop can continue in 
    fn get_compatible_directions(
        &self,
        pipe_type: PipeType,
        pipe_maze: &Vec<&str>) -> CompatibleDirections {
        let compatible_directions: CompatibleDirections = match pipe_type {
          PipeType::NS => [AdjacentDirection::NORTH, AdjacentDirection::SOUTH],
          PipeType::EW => [AdjacentDirection::WEST, AdjacentDirection::EAST],
          PipeType::NE => [AdjacentDirection::NORTH, AdjacentDirection::EAST],
          PipeType::NW => [AdjacentDirection::WEST, AdjacentDirection::NORTH],
          PipeType::SW => [AdjacentDirection::SOUTH, AdjacentDirection::WEST],
          PipeType::SE => [AdjacentDirection::EAST, AdjacentDirection::SOUTH],

          // special case 'S' (starting position):
          // all 4 directions are possible directions to step to
          PipeType::S  => self.get_compatible_directions(
            self.get_compatible_directions_from_start(pipe_maze), pipe_maze),
        };

        return compatible_directions;
    }

    // special case: start pipe/position can have adjacent pipes in
    // all directions (North, South, East, West), but can at most have
    // 2 adjacent types; to find them, we need to check all directions
    fn get_compatible_directions_from_start(
        &self, pipe_maze: &Vec<&str>) -> PipeType {
        assert!(self.pipe_type == PipeType::S);

        let mut compatible_directions: Vec<AdjacentDirection> = Vec::new();

        let north_pipe: Option<PipeType> = get_pipe_type(
            self.row, self.col, pipe_maze, AdjacentDirection::NORTH);
        let south_pipe: Option<PipeType> = get_pipe_type(
            self.row, self.col, pipe_maze, AdjacentDirection::SOUTH);
        let east_pipe: Option<PipeType> = get_pipe_type(
            self.row, self.col, pipe_maze, AdjacentDirection::EAST);
        let west_pipe: Option<PipeType> = get_pipe_type(
            self.row, self.col, pipe_maze, AdjacentDirection::WEST);

        match north_pipe {
            Some(pipe_type) => match pipe_type {
                PipeType::NS | PipeType::SE | PipeType::SW =>
                    compatible_directions.push(AdjacentDirection::NORTH),
                _ => (),
            }
            None => (),
        }

        match south_pipe {
            Some(pipe_type) => match pipe_type {
                PipeType::NS | PipeType::NE | PipeType::NW =>
                    compatible_directions.push(AdjacentDirection::SOUTH),
                _ => (),
            }
            None => (),
        }

        match west_pipe {
            Some(pipe_type) => match pipe_type {
                PipeType::EW | PipeType::NE | PipeType::SE =>
                    compatible_directions.push(AdjacentDirection::WEST),
                _ => (),
            }
            None => (),
        }

        match east_pipe {
            Some(pipe_type) => match pipe_type {
                PipeType::EW | PipeType::NW | PipeType::SW =>
                    compatible_directions.push(AdjacentDirection::EAST),
                _ => (),
            }
            None => (),
        }

        assert!(compatible_directions.len() == MAX_ADJACENT_PIPES);

        if compatible_directions.contains(&AdjacentDirection::NORTH) &&
           compatible_directions.contains(&AdjacentDirection::SOUTH) {
            return PipeType::NS;
        } else if compatible_directions.contains(&AdjacentDirection::EAST) &&
                  compatible_directions.contains(&AdjacentDirection::WEST) {
         return PipeType::EW;
        } else if compatible_directions.contains(&AdjacentDirection::NORTH) &&
                  compatible_directions.contains(&AdjacentDirection::WEST) {
         return PipeType::NW;
        } else if compatible_directions.contains(&AdjacentDirection::SOUTH) &&
                  compatible_directions.contains(&AdjacentDirection::WEST) {
         return PipeType::SW;
        } else if compatible_directions.contains(&AdjacentDirection::SOUTH) &&
                  compatible_directions.contains(&AdjacentDirection::EAST) {
         return PipeType::SE;
        } else {
            assert!(false, "Start pipe doesn't have 2 adjacent pipes!");
            return PipeType::S;
        }
    }

}

#[inline(always)]
fn find_next_pipe(
    current_pipe: &LoopPipe,
    pipe_maze: &Vec<&str>,
    direction: &LoopDirection) -> LoopPipe {
    return current_pipe.get_adjacent_pipes(pipe_maze)[*direction];
}


// just for debugging
#[cfg(debug_assertions)]
fn find_pipe_loop(
    start_pipe: LoopPipe,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> Vec<LoopPipe> {
    let mut pipe_loop: Vec<LoopPipe> = Vec::new();

    // move to the first adjacent pipe of the starting pipe/position
    let mut current_pipe: LoopPipe =
        find_next_pipe(&start_pipe, &pipe_maze, &direction);

    let max_loop_iterations: u32 =
        (pipe_maze.len() * pipe_maze[0].len()) as u32;
    let mut loop_iterations: u32 = 0;

    // keep moving to the next adjacent pipe until the loop is
    // completed and we are back at the starting pipe/position
    while current_pipe != start_pipe {
        current_pipe = find_next_pipe(&current_pipe, &pipe_maze, &direction);
        println!("next pipe after start: {:?}", current_pipe);
        std::process::exit(0);
        pipe_loop.push(current_pipe);

        loop_iterations += 1;

        // catch infinite loops for debugging purposes (or wrong input)
        if loop_iterations > max_loop_iterations {
            assert!(false, "Pipe loop can't be found!");
        }
    }

    return pipe_loop;
}

// find the single pipe loop starting from the start pipe in the pipe maze
#[cfg(not(debug_assertions))]
fn find_pipe_loop(
    start_pipe: LoopPipe,
    pipe_maze: Vec<&str>,
    direction: LoopDirection) -> Vec<LoopPipe> {
    let mut pipe_loop: Vec<LoopPipe> = Vec::new();

    // move to the first adjacent pipe of the starting pipe/position
    let mut current_pipe: LoopPipe =
        find_next_pipe(&start_pipe, &pipe_maze, &direction);

    // keep moving to the next adjacent pipe until the loop is
    // completed and we are back at the starting pipe/position
    while current_pipe != start_pipe {
        current_pipe = find_next_pipe(&current_pipe, &pipe_maze, &direction);
        pipe_loop.push(current_pipe);
    }

    return pipe_loop;
}

// calculate the furthest distance from the starting position/pipe
// when walking through the pipe loop
fn calc_furthest_distance_from_start(pipe_loop: Vec<LoopPipe>) -> u32 {
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

fn get_pipe_type(
    row_idx: usize,
    col_idx: usize,
    pipe_maze: &Vec<&str>,
    adjacent_direction: AdjacentDirection) -> Option<PipeType> {
    let max_row_idx: usize = pipe_maze.len() - 1;
    let max_col_idx: usize = pipe_maze[0].len() - 1;

    let mut adjacent_row_idx: usize = row_idx;
    let mut adjacent_col_idx: usize = col_idx;

    match adjacent_direction {
        AdjacentDirection::NORTH => if adjacent_row_idx > 0 {
            adjacent_row_idx -= 1;
        } else {
            return None;
        },
        AdjacentDirection::SOUTH => if adjacent_row_idx < max_row_idx {
            adjacent_row_idx += 1;
        } else {
            return None;
        },
        AdjacentDirection::EAST => if adjacent_col_idx < max_col_idx {
            adjacent_col_idx += 1;
        } else {
            return None;
        },
        AdjacentDirection::WEST => if adjacent_col_idx > 0 {
            adjacent_col_idx -= 1;
        } else {
            return None;
        },
    }

    let symbol: char =
        pipe_maze[adjacent_row_idx].chars().nth(adjacent_col_idx).unwrap();

    match symbol {
        '|' => Some(PipeType::NS),
        '-' => Some(PipeType::EW),
        'F' => Some(PipeType::SE),
        '7' => Some(PipeType::SW),
        'J' => Some(PipeType::NW),
        'L' => Some(PipeType::NE),
        'S' => Some(PipeType::S),
        _ => None,
    }
}
