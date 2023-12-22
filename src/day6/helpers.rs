const TIME_LINE_ID: &str = "Time:";
const DISTANCE_LINE_ID: &str = "Distance:";

pub fn get_times(file_content: &str) -> Vec<u64> {
    let first_line_end_idx: usize = file_content.find("\n").unwrap();

    let times_str: &str = &file_content[TIME_LINE_ID.len()..first_line_end_idx];

    let times: Vec<u64> = get_numbers_from_line(times_str);

    return times;
}

pub fn get_time(file_content: &str) -> u64 {
    let line_start_idx: usize = file_content.find(TIME_LINE_ID).unwrap();
    let line_end_idx: usize = file_content.find("\n").unwrap();

    let distance_line: &str =
        &file_content[line_start_idx+TIME_LINE_ID.len()..line_end_idx];

    return get_number_from_line(distance_line);
}

pub fn get_distances(file_content: &str) -> Vec<u64> {
    let first_line_end_idx: usize = file_content.find("\n").unwrap();
    let second_line_start_idx: usize =
        first_line_end_idx + 1 + DISTANCE_LINE_ID.len();

    let distances_str: &str = &file_content[second_line_start_idx..];

    let distances: Vec<u64> = get_numbers_from_line(distances_str);

    return distances;
}

pub fn get_distance(file_content: &str) -> u64 {
    let line_start_idx: usize = file_content.find(DISTANCE_LINE_ID).unwrap();
    let line_end_idx: usize = file_content.len();

    let distance_line: &str =
        &file_content[line_start_idx+DISTANCE_LINE_ID.len()..line_end_idx];

    return get_number_from_line(distance_line);
}

pub fn calc_num_of_record_possibilities(
    total_time: u64, record_distance: u64) -> u64 {


    let longest_possible_charge_time: u64 =
        calc_longest_possible_charge_time(total_time, record_distance);
    
    let shortest_possible_charge_time: u64 =
        calc_shortest_possible_charge_time(total_time, record_distance);

    // range of possible charging times to still beat the record distance
    // (i.e. total number of possibilites to beat the record distance)
    // is inclusive, so need to add 1 to the result here
    return longest_possible_charge_time - shortest_possible_charge_time + 1;
}

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    let nums: Vec<u64> = line.split(" ").map(
        |n| n.trim()).filter(
                |n| !n.is_empty()).map(
                    |n| n.parse().unwrap()).collect();

    return nums;
}

fn get_number_from_line(line: &str) -> u64 {
    let total_time: u64 = line.replace(" ", "").parse().unwrap();

    return total_time;
}

#[inline(always)]
fn calc_traveled_distance(time_remaining: u64, speed: u64) -> u64 {
    return speed * time_remaining;
}

// find longest/maximum possible charge time using binary search
fn calc_longest_possible_charge_time(
    total_time: u64, record_distance: u64) -> u64 {
    
    // look in range from middle to max time and try to find the charge time
    // that produces a speed that is just too small to beat the record time
    let mut low_time: u64 = total_time / 2;
    let mut high_time: u64 = total_time - 1;

    loop {
        let middle: u64 = (high_time + low_time) / 2;

        let lower_distance: u64 =
            calc_traveled_distance(total_time - middle, middle);
        let upper_distance: u64 =
            calc_traveled_distance(total_time - (middle + 1), middle + 1);

        // check if we found the longest time the boat
        // can charge to still beat the record distance
        let found_longest_time: bool =
          lower_distance > record_distance && upper_distance <= record_distance;

        if found_longest_time {
            return middle;
        }

        // if true, we're still in haven't reached the maximum possible
        // charging time and need to keep increasing it
        if lower_distance > record_distance &&
           upper_distance > record_distance {
            low_time = middle;
        // if true, we've jumped too far and skipped the maximum possible
        // charge time, so we need to decrease the time window again
        } else if lower_distance <= record_distance && 
                  upper_distance <= record_distance {
            high_time = middle;
        // if true, we've also jumped too far (see condition above)
        } else if lower_distance <= record_distance &&
                  upper_distance > record_distance {
            high_time = middle;
        }
    }
}

// find shortest/minimum possible charge time using binary search
fn calc_shortest_possible_charge_time(
    total_time: u64, record_distance: u64) -> u64 {
    let mut low_time: u64 = 1;
    let mut high_time: u64 = total_time / 2;

    loop {
        let middle: u64 = (high_time + low_time) / 2;

        let lower_distance: u64 =
            calc_traveled_distance(total_time - (middle - 1), middle - 1);
        let upper_distance: u64 =
            calc_traveled_distance(total_time - middle, middle);

            // check if we found the shortest time the boat
            // can charge to still beat the record distance
            let found_shortest_time: bool =
                lower_distance <= record_distance &&
                upper_distance > record_distance;
    
            if found_shortest_time {
                return middle;
            }
    
            // if true, we've jumped too far and skipped the minimum possible
            // charge time, so we need to increase the time window again
            if lower_distance <= record_distance &&
               upper_distance <= record_distance {
                low_time = middle;
            // if true, we've skipped the minimum possible charge time
            // and need to decrease the time window again
            } else if lower_distance > record_distance && 
                      upper_distance <= record_distance {
                high_time = middle;
            // same case as condition above; just jumped even further here
            } else if lower_distance > record_distance &&
                      upper_distance > record_distance {
                high_time = middle;
            }
    }
}
