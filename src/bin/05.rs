advent_of_code::solution!(5);

#[derive(Debug)]
pub struct Map {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

pub fn populate_map(input: Vec<&str>) -> Vec<Map> {
    let mut map: Vec<Map> = Vec::new();
    println!("{:?}", input);
    for i in 1..input.len() - 1 {
        let text: Vec<&str> = input[i].split(" ").collect();
        map.push(Map {
            destination_start: text[0].parse().unwrap(),
            source_start: text[1].parse().unwrap(),
            length: text[2].parse().unwrap(),
        });
    }
    map
}

pub fn search_destination(input: u64, map: &Vec<Map>, title: &str) -> u64 {
    let mut destination: u64 = 0;
    for i in 0..map.len() {
        let start = map[i].source_start;
        let stop = map[i].source_start + map[i].length;
        let mut counter: u64 = 0;
        for j in start..stop {
            if input == j {
                destination = map[i].destination_start + counter;
                break;
            }
            counter += 1;
        }
    }
    if destination == 0 {
        destination = input
    };
    println!("{}: {}", title, destination);
    destination
}

pub fn part_one(input: &str) -> Option<u64> {
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let temp_seeds: Vec<&str> = splitted_input[0].split(':').collect();
    let seeds_input: Vec<u64> = temp_seeds[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let seed_to_soil_input: Vec<&str> = splitted_input[1].split('\n').collect();
    let seed_to_soil: Vec<Map> = populate_map(seed_to_soil_input);
    let soil_to_fertilizer_input: Vec<&str> = splitted_input[2].split('\n').collect();
    let soil_to_fertilizer: Vec<Map> = populate_map(soil_to_fertilizer_input);
    let fertilizer_to_water_input: Vec<&str> = splitted_input[3].split('\n').collect();
    let fertilizer_to_water: Vec<Map> = populate_map(fertilizer_to_water_input);
    let water_to_light_input: Vec<&str> = splitted_input[4].split('\n').collect();
    let water_to_light: Vec<Map> = populate_map(water_to_light_input);
    let light_to_temperature_input: Vec<&str> = splitted_input[5].split('\n').collect();
    let light_to_temperature: Vec<Map> = populate_map(light_to_temperature_input);
    let temperature_to_humidity_input: Vec<&str> = splitted_input[6].split('\n').collect();
    let temperature_to_humidity: Vec<Map> = populate_map(temperature_to_humidity_input);
    let humidity_to_location_input: Vec<&str> = splitted_input[7].split('\n').collect();
    let humidity_to_location: Vec<Map> = populate_map(humidity_to_location_input);
    let mut soils: Vec<u64> = Vec::new();
    for seed in seeds_input {
        soils.push(search_destination(seed, &seed_to_soil, "soil"));
    }
    let fertilizers: Vec<u64> = soils
        .iter()
        .map(|x| search_destination(*x, &soil_to_fertilizer, "fertilizer"))
        .collect();
    let waters: Vec<u64> = fertilizers
        .iter()
        .map(|x| search_destination(*x, &fertilizer_to_water, "water"))
        .collect();
    let lights: Vec<u64> = waters
        .iter()
        .map(|x| search_destination(*x, &water_to_light, "light"))
        .collect();
    let temperatures: Vec<u64> = lights
        .iter()
        .map(|x| search_destination(*x, &light_to_temperature, "temperature"))
        .collect();
    let humidities: Vec<u64> = temperatures
        .iter()
        .map(|x| search_destination(*x, &temperature_to_humidity, "humidity"))
        .collect();
    let locations: Vec<u64> = humidities
        .iter()
        .map(|x| search_destination(*x, &humidity_to_location, "location"))
        .collect();
    println!("{:?}", locations);

    let mut min_location = u64::MAX;
    for loc in locations {
        if loc < min_location {
            min_location = loc;
        }
    }
    Some(min_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
