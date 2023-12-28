// parse all numbers from a line and store them in a vector
pub fn parse_nums_from_line(line: &str) -> Vec<i32> {
    const NUM_DELIMITER: &str = " ";

    let mut nums: Vec<i32> = Vec::new();

    for num in line.split(NUM_DELIMITER) {
        assert!(num.parse::<i32>().is_ok());
        nums.push(num.parse().unwrap());
    }

    return nums;
}

// find the next value for a history of numbers
// (for space efficiency, nums vector is modified in place
//  until the differences between adjacent numbers is 0)
pub fn find_next_val_of_history(mut nums: Vec<i32>) -> i32 {
    // check if all numbers in the provided vector (range) are 0
    let all_nums_zero =
        |vec: &Vec<i32>, n: usize| (&vec[0..n]).iter().all(|n| *n == 0);

    let total_nums: usize = nums.len();
    let mut unprocessed_nums_end_idx: usize = total_nums;

    let mut last_vals: Vec<i32> = vec![nums[total_nums - 1]];

    // loop until all adjacent number differences are 0
    while !all_nums_zero(&nums, unprocessed_nums_end_idx) {
        let last_val: i32 =
            calc_nums_difference(&mut nums[0..unprocessed_nums_end_idx]);

        last_vals.push(last_val);

        unprocessed_nums_end_idx = total_nums - last_vals.len() + 1;
    }

    // extrapolate the history
    let next_val_of_history: i32 = extrapolate_history(last_vals);

    return next_val_of_history;
}

// calculate the adjacent differences of all numbers in the
// provided vector slice (in-place)
fn calc_nums_difference(nums: &mut [i32]) -> i32 {
    let idx_of_last_val: usize = nums.len() - 2;

    for i in 0..nums.len() - 1 {
        nums[i] = nums[i+1] - nums[i];
    }

    return nums[idx_of_last_val];

}

// extrapolate the history (i.e. predict the next number of the history)
fn extrapolate_history(last_vals: Vec<i32>) -> i32 {
    let mut last_vals_iter = last_vals.iter().rev();
    let mut prev: i32 = *last_vals_iter.next().unwrap();
    let mut next_val: i32 = 0;

    for val in last_vals_iter {
        next_val = val + prev;
        prev = next_val;
    }

    return next_val;
}
