advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for text in splitted_input {
        let mut numbers = Vec::new();
        let char_list: Vec<char> = text.chars().collect();
        for c in char_list {
            if c.is_digit(10) {
                numbers.push(c);
            }
        }
        let string: String = format!("{}{}", numbers[0], numbers.last().unwrap());
        sum += string.parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for text in splitted_input {
        let mut numbers = Vec::new();
        let mut positions = Vec::new();
        let mut new_text = text.to_string();

        let words = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for (i, word) in words.iter().enumerate() {
            while let Some(start) = new_text.find(word) {
                numbers.push((i as u32).to_string());
                positions.push(start);
                new_text.remove(start + 1);
            }
        }
        let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for (i, digit) in digits.iter().enumerate() {
            while let Some(start) = new_text.find(digit) {
                numbers.push((i as u32).to_string());
                positions.push(start);
                new_text.remove(start);
            }
        }
        let mut permutation: Vec<usize> = (0..positions.len()).collect();
        permutation.sort_by(|a, b| positions[*a].cmp(&positions[*b]));
        let ordered_numbers: Vec<_> = permutation.iter().map(|i| &numbers[*i]).collect();
        let mut string = String::new();
        if ordered_numbers.len() == 1 {
            string = format!("{}", ordered_numbers[0]);
        } else if ordered_numbers.len() == 0 {
            println!("posizione vuota");
        } else {
            string = format!("{}{}", ordered_numbers[0], ordered_numbers.last().unwrap());
            //println!("{}", string);
        }
        //println!("{:?}\n{}", text, string);
        sum += string.parse::<u32>().unwrap();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
