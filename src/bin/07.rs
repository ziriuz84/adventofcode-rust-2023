use std::cmp::Ordering;
use std::collections::HashMap;
advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Hand<'a> {
    plays: &'a str,
    bid: u64,
    hand_type: HandType,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_card_value(c: char) -> u64 {
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

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let strenghts = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'J', 'T', 'Q', 'K', 'A',
        ];
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let mut self_strengths = Vec::new();
                let mut other_strengths = Vec::new();
                for c in self.plays.chars() {
                    self_strengths.push(strenghts.iter().position(|&ch| ch == c).unwrap());
                }
                for c in other.plays.chars() {
                    other_strengths.push(strenghts.iter().position(|&ch| ch == c).unwrap());
                }
                self_strengths.iter().cmp(other_strengths.iter())
            }
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Eq for Hand<'a> {}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self.plays == *other.plays
    }
}

fn get_hand_type(cards: &str) -> HandType {
    let mut counts: HashMap<char, u64> = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let first = cards.chars().next().unwrap();
    if counts.len() == 1 {
        HandType::FiveOfAKind
    } else if counts.len() == 2 {
        if [1, 4].contains(&counts[&first]) {
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
}

pub fn part_one(input: &str) -> Option<u64> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut cards: Vec<Hand> = Vec::new();
    for text in splitted_input {
        let temp_text: Vec<&str> = text.split_whitespace().collect();
        cards.push(Hand {
            plays: temp_text[0],
            bid: temp_text[1].parse::<u64>().unwrap(),
            hand_type: get_hand_type(temp_text[0]),
        });
    }
    cards.sort();
    for card in cards {
        println!("{:?}", card);
    }
    /*
    let sum = cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i as u64 + 1) * card.bid);
    Some(sum)
    */
    None
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
