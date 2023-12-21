use core::num;

pub fn get_times(file_content: &str) -> Vec<u32> {
    const TIME_LINE_ID: &str = "Time:";

    let first_line_end_idx: usize = file_content.find("\n").unwrap();

    let times_str: &str = &file_content[TIME_LINE_ID.len()..first_line_end_idx];

    let times: Vec<u32> = get_numbers_from_line(times_str);

    return times;
}

pub fn get_distances(file_content: &str) -> Vec<u32> {
    const DISTANCE_LINE_ID: &str = "Distance:";

    let first_line_end_idx: usize = file_content.find("\n").unwrap();
    let second_line_start_idx: usize =
        first_line_end_idx + 1 + DISTANCE_LINE_ID.len();

    let distances_str: &str = &file_content[second_line_start_idx..];

    let distances: Vec<u32> = get_numbers_from_line(distances_str);

    return distances;
}

pub fn calc_num_of_record_possibilities(
    total_time: u32, record_distance: u32) -> u32 {
    let mut num_of_record_possibilities: u32 = 0;

    for time in 1..total_time-1 {
        let speed: u32 = time;  // speed of boat in mm/ms

        // distance traveled in mm
        let distance_traveled: u32 =
            calc_traveled_distance(total_time - time, speed);

        if distance_traveled > record_distance {
            num_of_record_possibilities += 1;
        }
    }

    return num_of_record_possibilities;
}

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    let nums: Vec<u32> = line.split(" ").map(
        |n| n.trim()).filter(
                |n| !n.is_empty()).map(
                    |n| n.parse().unwrap()).collect();

    return nums;
}

#[inline(always)]
fn calc_traveled_distance(time_remaining: u32, speed: u32) -> u32 {
    return speed * time_remaining;
}