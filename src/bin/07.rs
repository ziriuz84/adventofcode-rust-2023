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

pub fn get_card_value(c: char, wildcards: bool) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match wildcards {
            false => 11,
            true => 1,
        },
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
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
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
    let sum = cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i as u64 + 1) * card.bid);
    for card in cards {
        println!("{:?}", card);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part_one(input);
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_b() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
        let result = part_one(input);
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_one_c() {
        let input = "627Q8 1
A26Q7 1
2K637 1";
        let result = part_one(input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_d() {
        let input = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43";
        let result = part_one(input);
        assert_eq!(result, Some(1343));
    }

    #[test]
    fn test_part_one_e() {
        let input = "23456 22
56789 19
KJJKK 2
AAAAJ 3
JJ243 7
QJ256 6
QQ562 5
Q8Q24 4
AAAAT 3
TJJJJ 2
6789T 18
789TJ 17
22345 13
34567 21
45678 20
32245 12
33245 11
89TJQ 16
9TJQK 15
TJQKA 14
3J245 10
J3425 9
J5432 8
JJJJJ 1";
        let result = part_one(input);
        assert_eq!(result, Some(2237));
    }

    #[test]
    fn test_part_one_f() {
        let input = "23456 1
AAAKK 3
AAAAA 1";
        let result = part_one(input);
        assert_eq!(result, Some(10));
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
