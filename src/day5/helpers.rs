use std::ops::Range;

const NUMBER_DELIMITER: &str = " ";
const END_OF_BLOCK_DELIMITER: &str = "\n";

// get all seeds for which we need to find the lowest location number
pub fn get_seeds(lines: &Vec<&str>) -> Vec<u32> {
    const SEED_ID: &str = "seeds: ";

    assert!(find_line_idx(SEED_ID, &lines).is_some());
    let seed_line_idx: usize = find_line_idx(SEED_ID, &lines).unwrap();
    let seed_line: &str = lines[seed_line_idx];

    let mut seeds: Vec<u32> = Vec::new();

    let seed_line_without_id: &str = seed_line.strip_prefix(SEED_ID).unwrap();

    for mut seed in seed_line_without_id.split(NUMBER_DELIMITER) {
        seed = seed.trim();
        let seed_num: u32 = seed.parse().unwrap();

        seeds.push(seed_num);
    }

    return seeds;
}

// read the seed-to-soil map
pub fn get_seed_to_soil_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const SEED_TO_SOIL_ID: &str = "seed-to-soil map:";
    return parse_map(SEED_TO_SOIL_ID, lines);
}

// read the soil-to-fertilizer map
pub fn get_soil_to_fertilizer_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const SOIL_TO_FERTILIZER_ID: &str = "soil-to-fertilizer map:";
    return parse_map(SOIL_TO_FERTILIZER_ID, lines);
}

// read the fertilizer-to-water map
pub fn get_fertilizer_to_water_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const FERTILIZER_TO_WATER_ID: &str = "fertilizer-to-water map:";
    return parse_map(FERTILIZER_TO_WATER_ID, lines);
}

// read the water-to-light map
pub fn get_water_to_light_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const WATER_TO_LIGHT_ID: &str = "water-to-light map:";
    return parse_map(WATER_TO_LIGHT_ID, lines);
}

// read the light-to-temperature map
pub fn get_light_to_temperature_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const LIGHT_TO_TEMPERATURE_ID: &str = "light-to-temperature map:";
    return parse_map(LIGHT_TO_TEMPERATURE_ID, lines);
}

// read the temperature-to-humidity map
pub fn get_temperature_to_humidity_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const TEMPERATURE_TO_HUMIDITY_ID: &str = "temperature-to-humidity map:";
    return parse_map(TEMPERATURE_TO_HUMIDITY_ID, lines);
}

// read the humidity-to-location map
pub fn get_humidity_to_location_map(lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    const HUMIDITY_TO_LOCATION_ID: &str = "humidity-to-location map:";
    return parse_map(HUMIDITY_TO_LOCATION_ID, lines);
}


// try to find a line with the provided line_id/prefixc in a
// vector of lines and return its index if it could be found
fn find_line_idx(line_id: &str, lines: &Vec<&str>) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with(&line_id) {
            return Some(i);
        }
    }

    return None;
}

fn parse_map(line_id: &str, lines: &Vec<&str>) ->
    (Vec<Range<u32>>, Vec<u32>) {
    assert!(find_line_idx(line_id, &lines).is_some());
    let map_start_idx: usize =
        find_line_idx(line_id, &lines).unwrap() + 1;
    
    let mut i: usize = map_start_idx;
    let mut line: &str = lines[i];

    let mut destination_ranges: Vec<Range<u32>> = Vec::new();
    let mut source_offsets: Vec<u32> = Vec::new();

    while line != END_OF_BLOCK_DELIMITER {
        let nums: Vec<u32> =
            line.trim().split(NUMBER_DELIMITER).map(
                |l| l.trim().parse().unwrap()).collect();

        assert!(nums.len() == 3);
        
        let range_len: u32 = nums[2];
        let destination_range_start: u32 = nums[0];
        let source_range_start: u32 = nums[1];

        source_offsets.push(source_range_start);
        destination_ranges.push(
            destination_range_start..destination_range_start+range_len);

        i += 1;
        line = lines[i];
    }

    return (destination_ranges, source_offsets);
}
