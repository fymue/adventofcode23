// Solutions to https://adventofcode.com/2023/day/2

mod helpers;

use crate::day2::helpers::get_gameid;
use crate::day2::helpers::is_game_possible;
use crate::day2::helpers::calc_power_of_minimum_cubeset;

pub fn puzzle1(file_content: String) -> String {
    let mut gameid_sum: u32 = 0;

    for mut game in file_content.split("\n") {
        game = game.trim();   // strip line of whitespace
        if game.is_empty() {  // skip empty lines
            continue;
        }

        let gameid: u32 = get_gameid(game);

        if is_game_possible(game) {
            gameid_sum += gameid;
        }
    }

    return gameid_sum.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let mut power_of_sets_sum: u32 = 0;

    for mut game in file_content.split("\n") {
        game = game.trim();   // strip line of whitespace
        if game.is_empty() {  // skip empty lines
            continue;
        }

        // calculate the power of the minimum set of cubes of the current game
        let power_of_minimum_cubeset: u32 = calc_power_of_minimum_cubeset(game);

        power_of_sets_sum += power_of_minimum_cubeset;
    }

    return power_of_sets_sum.to_string();
}
