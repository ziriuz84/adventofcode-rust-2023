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
        println!("{}", string);
        sum += string.parse::<u32>().unwrap();
    }
    println!("{}", sum);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for text in splitted_input {
        let mut numbers = String::new();
        let mut number_1: char = 'a';
        let mut number_2: char = 'a';
        let mut index_1: u32;
        let mut index_2: u32;
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("two") {
            if number_1 == 'a' {
                number_1 = '2';
                index_1 = text.find("two").unwrap() as u32;
            } else {
                number_2 = "2";
                index_2 = text.find("two").unwrap() as u32;
            }
        }
        if text.contains("three") {
            if number_1 == 'a' {
                number_1 = '2';
                index_1 = text.find("three").unwrap() as u32;
            } else {
                number_2 = "";
                index_2 = text.find("three").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
        if text.contains("one") {
            if number_1 == 'a' {
                number_1 = '1';
                index_1 = text.find("one").unwrap() as u32;
            } else {
                number_2 = "1";
                index_2 = text.find("one").unwrap() as u32;
            }
        }
    }
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
