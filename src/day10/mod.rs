mod helpers;
use helpers::GridPosition;

pub fn puzzle1(file_content: String) -> String {
    let pipe_maze: Vec<&str> = helpers::parse_pipe_maze(&file_content);

    assert!(helpers::find_start_pipe(&pipe_maze).is_some());
    let start_position: GridPosition =
      helpers::find_start_pipe(&pipe_maze).unwrap();
    
    let furthest_distance: u32 = helpers::find_furthest_distance_from_start(
        start_position, pipe_maze, helpers::LOOP_CLOCKWISE);
    
    return furthest_distance.to_string();
}
