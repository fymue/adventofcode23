use core::num;

mod helpers;

pub fn puzzle1(file_content: String) -> String {
    let mut ans: u32 = 1;

    let times: Vec<u32> = helpers::get_times(&file_content);
    
    let distances: Vec<u32> = helpers::get_distances(&file_content);

    for (i, time) in times.iter().enumerate() {
        let num_of_record_possibilites: u32 =
            helpers::calc_num_of_record_possibilities(*time, distances[i]);
        println!("num of p for {time} and {}, {}", distances[i], num_of_record_possibilites);
        ans *= num_of_record_possibilites;
    }

    return ans.to_string();
}
