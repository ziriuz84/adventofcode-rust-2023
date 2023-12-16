advent_of_code::solution!(9);

fn prepare_arrays(text: &str) -> Vec<Vec<i32>> {
    let mut arrays: Vec<Vec<i32>> = Vec::new();
    let temp: Vec<i32> = text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut new_temp: Vec<i32> = Vec::new();
    new_temp = temp.clone();
    arrays.push(temp);
    while !new_temp.iter().all(|&n| n == 0) {
        let mut new_numbers: Vec<i32> = Vec::new();
        for i in 0..(new_temp.len() - 1) {
            new_numbers.push(new_temp[i + 1] - new_temp[i]);
        }
        new_temp = new_numbers.clone();
        arrays.push(new_numbers);
    }
    arrays
}

pub fn part_one(input: &str) -> Option<i32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum: i32 = 0;
    for text in splitted_input {
        let mut arrays = prepare_arrays(text);
        let mut new_arrays = arrays.clone();
        let new_arrays_temp = arrays.clone();
        for i in (1..arrays.len()).rev() {
            arrays[i - 1]
                .push(new_arrays[i].last().unwrap() + new_arrays_temp[i - 1].last().unwrap());
            new_arrays = arrays.clone();
        }
        sum += arrays[0].last().unwrap();
        //println!("{:?}", arrays);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum: i32 = 0;
    for text in splitted_input {
        let mut arrays = prepare_arrays(text);
        let mut new_arrays = arrays.clone();
        let new_arrays_temp = arrays.clone();
        for i in (1..arrays.len()).rev() {
            if i == arrays.len() {
                arrays[i - 1].insert(0, 0)
            } else {
                arrays[i - 1].insert(
                    0,
                    new_arrays_temp[i - 1].first().unwrap() - new_arrays[i].first().unwrap(),
                );
                new_arrays = arrays.clone();
            }
            println!("{:?}", arrays);
        }
        sum += arrays[0].first().unwrap();
        //println!("{:?}", arrays);
    }
    Some(sum)
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
    fn test_two_wip() {
        let input = "10 13 16 21 30 45";
        let result = part_two(&input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
