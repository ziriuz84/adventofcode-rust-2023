use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input = input.lines().collect::<Vec<&str>>();
    let directions: Vec<char> = splitted_input[0].chars().collect();
    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    let mut steps: u32 = 0;
    for i in 2..splitted_input.len() {
        let temp: String = splitted_input[i]
            .chars()
            .filter(|&c| !c.is_whitespace() && c != '(' && c != ')')
            .collect();
        let splitted_temp: Vec<String> = temp.split('=').map(String::from).collect();
        let map: Vec<String> = splitted_temp[1]
            .clone()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        nodes.insert(splitted_temp[0].clone(), Vec::from(map));
    }
    let mut directions_counter: usize = 0;
    let mut current_node_key: String = String::from("");
    let mut new_node: String = String::from("");
    current_node_key = String::from("AAA");
    while current_node_key != "ZZZ" {
        let mut direction: u32 = 0;
        match directions[directions_counter] {
            'L' => direction = 0,
            'R' => direction = 1,
            _ => panic!("Wrong direction!"),
        }
        let current_node = nodes.get(&current_node_key).unwrap();
        current_node_key = current_node[direction as usize].clone();
        if directions_counter == directions.len() - 1 {
            directions_counter = 0;
        } else {
            directions_counter += 1;
        }
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_b() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part_one(input);
        assert_eq!(result, Some(6));
    }

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
