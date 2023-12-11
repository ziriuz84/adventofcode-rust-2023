use std::collections::HashMap;
advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Hand<'a> {
    plays: &'a str,
    bid: u32,
    hand_type: HandType,
}

#[derive(Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unimplemented!(),
    }
}

pub fn get_hand_type(cards: &str) -> HandType {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let first = cards.chars().next().unwrap();
    if counts.len() == 1 {
        return HandType::FiveOfAKind;
    } else if counts.len() == 2 {
        if vec![1, 4].contains(&counts[&first]) {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    } else if counts.len() == 3 {
        for c in cards.chars() {
            if counts[&c] == 3 {
                return HandType::ThreeOfAKind;
            }
        }
        return HandType::TwoPair;
    } else if counts.len() == 4 {
        return HandType::Pair;
    } else {
        return HandType::HighCard;
    }
    return HandType::HighCard;
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    println!("{:?}", splitted_input);
    let mut cards: Vec<Hand> = Vec::new();
    let sum = 0;
    for text in splitted_input {
        let temp_text: Vec<&str> = text.split_whitespace().collect();
        cards.push(Hand {
            plays: temp_text[0],
            bid: temp_text[1].parse::<u32>().unwrap(),
            hand_type: get_hand_type(temp_text[0]),
        });
    }
    println!("{:?}", cards);
    Some(sum)
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
