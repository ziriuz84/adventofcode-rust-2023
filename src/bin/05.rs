advent_of_code::solution!(5);

pub struct Map {
    destination_start: u32,
    source_start: u32,
    length: u32,
    destination_range: Vec<u32>,
    source_range: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    println!("{:?}", splitted_input);
    let seeds_input = splitted_input[0];
    let seed_to_soil_input: Vec<&str> = splitted_input[1].split('\n').collect();
    let soil_to_fertilizer_input: Vec<&str> = splitted_input[2].split('\n').collect();
    let fertilizer_to_water_input: Vec<&str> = splitted_input[3].split('\n').collect();
    let water_to_light_input: Vec<&str> = splitted_input[4].split('\n').collect();
    let light_to_temperature_input: Vec<&str> = splitted_input[5].split('\n').collect();
    let temperature_to_humidity_input: Vec<&str> = splitted_input[6].split('\n').collect();
    let humidity_to_location_input: Vec<&str> = splitted_input[7].split('\n').collect();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
