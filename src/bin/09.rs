advent_of_code::solution!(9);

fn new_line(input: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();
    for i in (1..input.len()).rev() {
        temp.push(input[i] - input[i - 1]);
    }
    println!("{:?}", temp);
    temp.reverse();
    println!("{:?}", temp);
    temp
}
pub fn part_one(input: &str) -> Option<i32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let sum: i32 = 0;
    for text in splitted_input {
        let mut arrays: Vec<Vec<i32>> = Vec::new();
        let temp: Vec<i32> = text
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut new_temp: Vec<i32> = Vec::new();
        new_temp = temp.clone();
        arrays.push(temp);
        while new_temp.iter().all(|&n| n >= 0) {
            arrays.push(new_temp.clone());
            new_temp = new_temp
                .as_slice()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            println!("{:?}", arrays);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_wip() {
        let input = "0 3 6 9 12 15";
        let result = part_one(&input);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_one() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part_one(&input);
        assert_eq!(result, Some(114));
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
