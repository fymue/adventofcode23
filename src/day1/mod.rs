use std::{fs::File, io::Read};

const FILE_RADIX: u32 = 10; // digits in file are decimal numbers (Base10)

pub fn day1_puzzle1(input_file_path: &str) -> String {
    let mut input_file: File =
        File::open(input_file_path).expect(
            &format!("Couldn't open input file '{}'!", input_file_path));

    let mut file_content: String = String::new();
    input_file
        .read_to_string(&mut file_content).expect(
            &format!("Couldn't read input file '{}'!", input_file_path));

    let mut total_sum: u32 = 0;

    for mut line in file_content.split("\n") {
        println!("{}", line);
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut num_digits: [char; 2] = ['\0'; 2];

        num_digits[0] = get_number(line.chars()).unwrap();
        num_digits[1] = get_number(line.chars().rev()).unwrap();

        let num_str: String = String::from_iter(num_digits);
        let num: u32 = num_str.parse().unwrap();
        total_sum += num;
    }

    return total_sum.to_string();
}

fn get_number(it: impl Iterator<Item = char>) -> Result<char, &'static str> {
    for chr in it {
        if chr.is_digit(FILE_RADIX) {
            return Ok(chr);
        }
    }

    return Err("Couldn't find any number in the provided Char iterator!");
}

