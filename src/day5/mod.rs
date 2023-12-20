use std::ops::Range;

mod helpers;

// TODO: just save offset instead of destination range
// iterate through all ranges of map and check if key is contained
// in any of the ranges; if not, simply pass same key to next map;
// if so, add offset to key and pass to next map

pub fn puzzle1(file_content: String) -> String {
    let lines: Vec<&str> =
        file_content.split("\n").map(|l| l.trim()).filter(
            |l| !l.is_empty()).collect();

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }
    }

    let seeds: Vec<u32> = helpers::get_seeds(&lines);

    let seed_to_soil_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_seed_to_soil_map(&lines);
    
    let soil_to_fertilizer_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_soil_to_fertilizer_map(&lines);

    let fertilizer_to_water_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_fertilizer_to_water_map(&lines);
    
    let water_to_light_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_water_to_light_map(&lines);
    
    let light_to_temperature_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_light_to_temperature_map(&lines);

    let temperature_to_humidity_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_temperature_to_humidity_map(&lines);

    let humidity_to_location_map: (Vec<Range<u32>>, Vec<u32>) =
        helpers::get_humidity_to_location_map(&lines);

    let lowest_location: u32 = 0;

    return lowest_location.to_string();
}
