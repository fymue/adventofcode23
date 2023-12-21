use std::ops::Range;

mod helpers;

pub fn puzzle1(file_content: String) -> String {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let seeds: Vec<u64> = helpers::get_seeds(&lines);

    let seed_to_soil_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_seed_to_soil_map(&lines);
    
    let soil_to_fertilizer_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_soil_to_fertilizer_map(&lines);

    let fertilizer_to_water_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_fertilizer_to_water_map(&lines);
    
    let water_to_light_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_water_to_light_map(&lines);
    
    let light_to_temperature_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_light_to_temperature_map(&lines);

    let temperature_to_humidity_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_temperature_to_humidity_map(&lines);

    let humidity_to_location_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_humidity_to_location_map(&lines);

    let mut lowest_location: u64 = u64::MAX;

    for seed in seeds {
        let soil_number: u64 = helpers::map_number(
            seed, &seed_to_soil_map);

        let fertilizer_number: u64 = helpers::map_number(
            soil_number, &soil_to_fertilizer_map);

        let water_number: u64 = helpers::map_number(
            fertilizer_number, &fertilizer_to_water_map);
    
        let light_number: u64 = helpers::map_number(
            water_number, &water_to_light_map);

        let temperature_number: u64 = helpers::map_number(
            light_number, &light_to_temperature_map);
        
        let humidity_number: u64 = helpers::map_number(
            temperature_number, &temperature_to_humidity_map);

        let location_number: u64 = helpers::map_number(
            humidity_number, &humidity_to_location_map);

        if location_number < lowest_location {
            lowest_location = location_number;
        }
    }

    return lowest_location.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let seed_ranges: Vec<Range<u64>> = helpers::get_seed_ranges(&lines);

    let seed_to_soil_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_seed_to_soil_map(&lines);
    
    let soil_to_fertilizer_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_soil_to_fertilizer_map(&lines);

    let fertilizer_to_water_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_fertilizer_to_water_map(&lines);
    
    let water_to_light_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_water_to_light_map(&lines);
    
    let light_to_temperature_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_light_to_temperature_map(&lines);

    let temperature_to_humidity_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_temperature_to_humidity_map(&lines);

    let humidity_to_location_map: (Vec<i64>, Vec<Range<u64>>) =
        helpers::get_humidity_to_location_map(&lines);

    let mut lowest_location: u64 = u64::MAX;

    for seed_range in seed_ranges {
        for seed in seed_range {
            let soil_number: u64 = helpers::map_number(
                seed, &seed_to_soil_map);

            let fertilizer_number: u64 = helpers::map_number(
                soil_number, &soil_to_fertilizer_map);

            let water_number: u64 = helpers::map_number(
                fertilizer_number, &fertilizer_to_water_map);
        
            let light_number: u64 = helpers::map_number(
                water_number, &water_to_light_map);

            let temperature_number: u64 = helpers::map_number(
                light_number, &light_to_temperature_map);
            
            let humidity_number: u64 = helpers::map_number(
                temperature_number, &temperature_to_humidity_map);

            let location_number: u64 = helpers::map_number(
                humidity_number, &humidity_to_location_map);

            if location_number < lowest_location {
                lowest_location = location_number;
            }
        }
    }

    return lowest_location.to_string();
}
