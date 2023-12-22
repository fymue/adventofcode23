mod helpers;

pub fn puzzle1(file_content: String) -> String {
    let mut ans: u64 = 1;

    let file_content: &str = file_content.trim();

    let times: Vec<u64> = helpers::get_times(file_content);
    
    let distances: Vec<u64> = helpers::get_distances(file_content);

    for (i, time) in times.iter().enumerate() {
        let num_of_record_possibilites: u64 =
            helpers::calc_num_of_record_possibilities(*time, distances[i]);
        ans *= num_of_record_possibilites;
    }

    return ans.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let file_content: &str = file_content.trim();

    let total_time: u64 = helpers::get_time(file_content);

    let record_distance: u64 = helpers::get_distance(file_content);

    let total_record_beating_races: u64 =
        helpers::calc_num_of_record_possibilities(total_time, record_distance);

    return total_record_beating_races.to_string();
}
