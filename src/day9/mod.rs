mod helpers;

pub fn puzzle1(file_content: String) -> String {
    let mut ans: i32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let nums: Vec<i32> = helpers::parse_nums_from_line(line);

        let at_beginning: bool = false;
        let next_val_of_history: i32 =
            helpers::find_next_val_of_history(nums, at_beginning);

        ans += next_val_of_history;
    }

    return ans.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let mut ans: i32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let nums: Vec<i32> = helpers::parse_nums_from_line(line);

        let at_beginning: bool = true;
        let next_val_of_history: i32 =
            helpers::find_next_val_of_history(nums, at_beginning);

        ans += next_val_of_history;
    }

    return ans.to_string();
}
