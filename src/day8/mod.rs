mod helpers;

use std::collections::HashMap;
use helpers::JumpNodes;

pub fn puzzle1(file_content: String) -> String {
    // parse the directions
    let directions: Vec<u8> = helpers::parse_directions(&file_content);

    // collect all existing nodes as well as their index
    // in the to-be-generated lookup array 
    let node_locations: HashMap<&str, usize> =
        helpers::map_node_locations(&file_content);

    // parse the network by recording the indices of all left/right jumps
    // of every node in the network vector
    let network: Vec<JumpNodes> =
        helpers::parse_network(&file_content, &node_locations);

    // calculate the number of steps it takes to go from "AAA" to "ZZZ"
    let total_steps: u32 = helpers::calc_total_steps_puzzle1(
        &directions, &node_locations, &network);

    return total_steps.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    // parse the directions
    let directions: Vec<u8> = helpers::parse_directions(&file_content);

    let (start_nodes, end_nodes): (Vec<usize>, Vec<usize>) =
        helpers::get_start_end_nodes(&file_content);

    // collect all existing nodes as well as their index
    // in the to-be-generated lookup array 
    let node_locations: HashMap<&str, usize> =
        helpers::map_node_locations(&file_content);

    // parse the network by recording the indices of all left/right jumps
    // of every node in the network vector
    let network: Vec<JumpNodes> =
        helpers::parse_network(&file_content, &node_locations);

    // calculate the number of steps it takes to go from all nodes
    // that end with 'A' to all nodes that end with 'Z'
    let total_steps: u64 = helpers::calc_total_steps_puzzle2(
        &directions, &network, start_nodes, end_nodes);

    return total_steps.to_string();
}
