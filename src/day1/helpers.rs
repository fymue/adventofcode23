const FILE_RADIX: u32 = 10; // digits in file are decimal numbers (Base10)

// all possible number words
const NUMBER_WORDS: [&str; 9] =
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// corresponding number characters to all possible number words
const NUMBER_CHAR_OF_WORD: [char; 9] =
    ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

// iterate over every character of line and return the
// first character that is a Base10 digit
pub fn get_first_number_char(line: impl Iterator<Item = char>) ->
    Result<u32, &'static str> {
        for chr in line {
            if chr.is_digit(FILE_RADIX) {
                return Ok(chr.to_digit(FILE_RADIX).unwrap());
            }
        }

        return Err("Couldn't find any number character in the provided line!");
}

// iterate over every character in line and return either the
// first number word or number character, depending on which comes first
pub fn get_first_number_word(line: &str) -> Result<u32, &'static str> {
    for (i, chr) in line.chars().enumerate() {
        if chr.is_alphabetic() {
            // if character is alphabetic, iterate over every number word
            // and check if the current character marks the start of the
            // current number word; if so, immediately return the
            // corresponding digit of the found number word
            for (j, number) in NUMBER_WORDS.iter().enumerate() {
                let word_end_idx: usize = i + number.len();
                let word_inbounds: bool = word_end_idx < line.len();

                if word_inbounds && *number == &line[i..word_end_idx] {
                    let num_char_of_word: char = NUMBER_CHAR_OF_WORD[j];
                    return Ok(num_char_of_word.to_digit(FILE_RADIX).unwrap());
                }
            }
        } else if chr.is_digit(FILE_RADIX) {
            // if a number character is found instead, return its
            // corresponding digit
            return Ok(chr.to_digit(FILE_RADIX).unwrap());
        }
    }

    return Err("Couldn't find any number word in the provided line!");
}

// iterate over every character in line in reverse order and return either the
// first number word or number character, depending on which comes first
pub fn get_last_number_word(line: &str) -> Result<u32, &'static str> {
    for (mut i, chr) in line.chars().rev().enumerate() {
        i = line.len() - 1 - i;

        if chr.is_alphabetic() {
            for (j, number) in NUMBER_WORDS.iter().enumerate() {
                let word_start_idx: isize =
                    i as isize - number.len() as isize + 1;

                let word_inbounds: bool = word_start_idx > 0;

                if word_inbounds && *number ==
                   &line[word_start_idx as usize..i+1] {
                    let num_char_of_word: char = NUMBER_CHAR_OF_WORD[j];
                    return Ok(num_char_of_word.to_digit(FILE_RADIX).unwrap());
                }
            }
        } else if chr.is_digit(FILE_RADIX) {
            return Ok(chr.to_digit(FILE_RADIX).unwrap());
        }
    }

    return Err("Couldn't find any number word in the provided line!");
}
