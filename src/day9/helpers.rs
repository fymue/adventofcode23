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
pub fn find_next_val_of_history(mut nums: Vec<i32>, at_beginning: bool) -> i32 {
    // check if all numbers in the provided vector (range) are 0
    let all_nums_zero =
        |vec: &Vec<i32>, n: usize| (&vec[0..n]).iter().all(|n| *n == 0);

    let total_nums: usize = nums.len();

    let mut relevant_vals: Vec<i32> = Vec::new();

    // we don't get the last number for the original number history
    // in the loop since the loop only iteratively processes the differences
    // between the numbers, so we need to add this number here
    if !at_beginning {
       relevant_vals.push(nums[total_nums - 1]);
    }

    let mut unprocessed_nums_end_idx: usize = total_nums;

    // loop until all adjacent number differences are 0
    while !all_nums_zero(&nums, unprocessed_nums_end_idx) {
        let unprocessed_nums: &mut [i32] =
            &mut nums[0..unprocessed_nums_end_idx];

        let mut relevant_val: i32 =
            get_relevant_val_at_beginning(unprocessed_nums);

        calc_nums_difference(unprocessed_nums);
 
        if !at_beginning {
            relevant_val = get_relevant_val_at_end(unprocessed_nums);
        }

        relevant_vals.push(relevant_val);

        unprocessed_nums_end_idx -= 1;
    }

    // extrapolate the history
    let next_val_of_history: i32 = if at_beginning {
        extrapolate_history_backward(relevant_vals)
    } else {
        extrapolate_history_forward(relevant_vals)
    };

    return next_val_of_history;
}

// calculate the adjacent differences of all numbers
// in the provided vector slice (in-place)
fn calc_nums_difference(nums: &mut [i32]) {
    for i in 0..nums.len() - 1 {
        nums[i] = nums[i+1] - nums[i];
    }
}

#[inline(always)]
fn get_relevant_val_at_beginning(nums: &[i32]) -> i32 {
    return nums[0];
}

#[inline(always)]
fn get_relevant_val_at_end(nums: &[i32]) -> i32 {
    return nums[nums.len() - 2];
}

// extrapolate the history (i.e. predict the next number of the history)
fn extrapolate_history_forward(relevant_vals: Vec<i32>) -> i32 {
    let mut relevant_vals_iter = relevant_vals.iter().rev();
    let mut prev: i32 = *relevant_vals_iter.next().unwrap();
    let mut next_val: i32 = 0;

    for val in relevant_vals_iter {
        next_val = val + prev;
        prev = next_val;
    }

    return next_val;
}

// extrapolate the history (i.e. predict the next number of the history)
fn extrapolate_history_backward(relevant_vals: Vec<i32>) -> i32 {
    let mut relevant_vals_iter = relevant_vals.iter().rev();
    let mut prev: i32 = *relevant_vals_iter.next().unwrap();
    let mut next_val: i32 = 0;

    for val in relevant_vals_iter {
        next_val = val - prev;
        prev = next_val;
    }

    return next_val;
}
