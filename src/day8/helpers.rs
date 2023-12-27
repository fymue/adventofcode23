use std::{collections::HashMap, thread::current};

pub type JumpNodes = [usize; 2];

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";
const LOCATION_DELIMITER: &str = " = (";

// parse the directions ('L's will be 0, 'R's will be 1)
pub fn parse_directions(file: &str) -> Vec<u8> {
    let mut directions: Vec<u8> = Vec::new();

    let end_of_directions_line_idx: usize = file.find("\n").unwrap();
    let directions_line: &str = &file[..end_of_directions_line_idx];

    // make sure that all characters of line are R (for right) or L (for left)
    assert!(directions_line.chars().all(|c| c == 'R' || c == 'L'));

    // push all 'L's as 0, all 'R's as 1
    for chr in directions_line.chars() {
        directions.push((chr == 'R') as u8);
    }

    return directions;
}

// read all nodes/locations from the input file
pub fn map_node_locations(file: &str) -> HashMap<&str, usize> {
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    let mut node_counter: usize = 0;

    for mut line in file.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let location_delimiter: Option<usize> = line.find(LOCATION_DELIMITER);
        let has_location: bool = location_delimiter.is_some();

        if has_location {
            let location_delimiter_idx: usize = location_delimiter.unwrap();
            let node: &str = &line[..location_delimiter_idx];

            nodes.insert(node, node_counter);
            node_counter += 1;
        }
    }

    assert!(nodes.contains_key(START_NODE));
    assert!(nodes.contains_key(END_NODE));

    return nodes;
}

// parse all nodes and record the index of every
// left/right jump of each node in the network vector
pub fn parse_network(
    file: &str, node_locations: &HashMap<&str, usize>) -> Vec<JumpNodes> {
    const JUMP_NODE_DELIMITER: &str = ", ";

    let mut network: Vec<JumpNodes> = Vec::new();

    for mut line in file.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let location_delimiter: Option<usize> = line.find(LOCATION_DELIMITER);
        let has_location: bool = location_delimiter.is_some();

        if has_location {
            let jump_nodes_start_idx: usize =
                location_delimiter.unwrap() + LOCATION_DELIMITER.len();
            let jump_nodes_end_idx: usize = line.len() - 1;

            // collect part between the braces
            let jump_nodes: &str =
                &line[jump_nodes_start_idx..jump_nodes_end_idx];
            
            assert!(jump_nodes.find(JUMP_NODE_DELIMITER).is_some());
            let jump_node_delim_idx: usize =
                jump_nodes.find(JUMP_NODE_DELIMITER).unwrap();
            
            let left_node: &str = &jump_nodes[..jump_node_delim_idx];
            let right_node: &str =
                &jump_nodes[jump_node_delim_idx+JUMP_NODE_DELIMITER.len()..];
            
            assert!(node_locations.contains_key(left_node));
            assert!(node_locations.contains_key(right_node));


            network.push(
                [node_locations[left_node], node_locations[right_node]]);
        }
    }

    return network;
}

pub fn calc_total_steps(
    directions: &Vec<u8>,
    node_locations: &HashMap<&str, usize>,
    network: &Vec<JumpNodes>) -> u32 {
    let mut total_steps: u32 = 0;

    let end_node: usize = get_destination_idx(node_locations);

    let mut current_node: usize = node_locations[START_NODE];

    // index to track the current direction (either left or right)
    let mut direction_idx: usize = 0;

    while current_node != end_node {
        // direction is either 0 ('L') or 1 ('R')
        let direction: usize = directions[direction_idx] as usize;

        // jump to next node location
        current_node = network[current_node][direction];
        total_steps += 1;

        direction_idx += 1;
        // if we've processed all direction steps, start from the beginning
        if direction_idx == directions.len() {
            direction_idx = 0;
        }
    }

    return total_steps;
}

// returns the index of the destination/end node "ZZZ" in the network vector
fn get_destination_idx(node_locations: &HashMap<&str, usize>) -> usize {
    assert!(node_locations.contains_key(END_NODE));
    return node_locations[END_NODE];
}
