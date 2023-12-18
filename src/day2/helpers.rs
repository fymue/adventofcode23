use std::collections::HashMap;

const GAME_ID_STR:       &str = "Game ";
const END_OF_GAMEID_STR: &str = ": ";

// get GameID number from line
pub fn get_gameid(game: &str) -> u32 {
    // find index of Game ID number x (ID look like: "Game x:")
    assert!(game.starts_with("Game "));
    assert!(game.find(END_OF_GAMEID_STR).is_some());

    // start index of x
    let gameid_start_idx: usize = GAME_ID_STR.len();

    // end idx of x
    let gameid_end_idx:   usize = game.find(END_OF_GAMEID_STR).unwrap();

    // extract GameID from string and convert it to an actual number
    let gameid: u32 =
        (&game[gameid_start_idx..gameid_end_idx]).parse().unwrap();

    return gameid;
}

pub fn strip_gameid(game: &str) -> &str {
    assert!(game.find(END_OF_GAMEID_STR).is_some());

    let end_idx_of_gameid: usize =
        game.find(END_OF_GAMEID_STR).unwrap() + END_OF_GAMEID_STR.len();
    let game_without_id: &str = &game[end_idx_of_gameid..];

    return game_without_id;
}

// get number of cubes and color of cubes for current color draw
pub fn get_draw_components(draw: &str) -> (u32, &str) {
    let draw_components: Vec<&str> = draw.split(" ").collect();

    let draw_num:    u32 = draw_components[0].parse().unwrap();
    let draw_color: &str = draw_components[1];

    return (draw_num, draw_color);
}

// returns whether or not the game is legal/possible
// (i.e. if no more cubes than the max. allowed number of cubes
//  for each color were drawn in any of the draws)
pub fn is_game_possible(game: &str) -> bool {
    const MAX_RED_CUBES:   u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES:  u32 = 14;

    let max_color_draws: HashMap<&str, u32> = HashMap::from([
        ("red", MAX_RED_CUBES),
        ("green", MAX_GREEN_CUBES),
        ("blue", MAX_BLUE_CUBES),
    ]);

    let game_without_id: &str = strip_gameid(game);

    // iterate over draws of current game (separated by "; ")
    for draw in game_without_id.split("; ") {
        // iterate over color draws of current draw (separated by ", ")
        for color_draw in draw.split(", ") {
            // get number of cubes and color of cubes for current color draw
            let (draw_num, draw_color): (u32, &str) =
                get_draw_components(color_draw);

            // check if number of drawn cubes is legal
            if draw_num > max_color_draws[draw_color] {
                return false;
            }
        }
    }

    return true;
}

pub fn calc_power_of_minimum_cubeset(game: &str) -> u32 {
    let mut power_of_minimum_cubeset: u32 = 1;

    let mut minimum_cubeset: HashMap<&str, u32> = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);

    // remove the GameID from the game string
    let game_without_id: &str = strip_gameid(game);

    // iterate over draws of current game (separated by "; ")
    for draw in game_without_id.split("; ") {
        // iterate over color draws of current draw (separated by ", ")
        for color_draw in draw.split(", ") {
            // get number of cubes and color of cubes for current color draw
            let (draw_num, draw_color): (u32, &str) =
                get_draw_components(color_draw);

            assert!(minimum_cubeset.contains_key(draw_color));

            // if the number of drawn cubes for the current color is bigger
            // than the previously biggest draw number, update it
            if draw_num > minimum_cubeset[draw_color] {
                minimum_cubeset.insert(draw_color, draw_num);
            }
        }
    }

    // iterate over all color keys and multiply them all,
    // which equals the "power" of the cube set
    for (_color, minimum_num) in minimum_cubeset {
        power_of_minimum_cubeset *= minimum_num;
    }

    return power_of_minimum_cubeset;
}
