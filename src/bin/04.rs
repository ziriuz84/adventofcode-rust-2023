advent_of_code::solution!(4);

#[derive(Debug, Clone)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

pub fn populate_card(line: &str) -> Option<Card> {
    let fields: Vec<&str> = line.split(':').collect();
    let first_field: Vec<&str> = fields[0].trim().split(' ').collect();
    let id = first_field[1].parse::<u32>().unwrap();
    let second_field: Vec<&str> = fields[1].split('|').collect();
    let winning_numbers: Vec<u32> = second_field[0]
        .trim()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let playing_numbers: Vec<u32> = second_field[1]
        .trim()
        .split(' ')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let card: Card = Card {
        id,
        winning_numbers,
        playing_numbers,
    };
    Some(card)
}

pub fn get_card_points(card: Card) -> Option<u32> {
    let mut points: u32 = 0;
    let counter: u32 = card
        .winning_numbers
        .iter()
        .filter(|wn| card.playing_numbers.contains(wn))
        .count() as u32;
    if counter == 0 {
        points = 0;
    } else if counter == 1 {
        points = 1;
    } else if counter > 1 {
        points = 1;
        for i in 1..counter {
            points *= 2;
        }
    }
    Some(points)
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for item in splitted_input {
        let text = item.replace("   ", " ").replace("  ", " ");
        let card = populate_card(&text);
        let points = get_card_points(card?);
        sum += points?;
    }

    Some(sum)
}

pub fn get_winning_cards(card: Card) -> usize {
    card.winning_numbers
        .iter()
        .filter(|wn| card.playing_numbers.contains(wn))
        .count()
}

pub fn get_total_cards(
    cards: &[Card],
    card: &Card,
    starting_number: usize,
    total_cards: &mut Vec<Card>,
) {
    /*
    cards
        .iter()
        .map(|card| get_winning_cards(card))
        .sum()
        */
    let this_card: Card = card.clone();
    let single_total_card: usize = get_winning_cards(this_card);
    let start_index: usize = starting_number + 1;
    println!("{} {} {}", starting_number, single_total_card, start_index);
    for i in start_index..=single_total_card + 1 {
        //if i < cards.len() {
        let actual_card: Card = cards[i].clone();
        let temp_card: Card = cards[i].clone();
        let mut total_card_copy = total_cards.clone();
        &total_cards.push(temp_card);
        get_total_cards(&cards, &actual_card, i, &mut total_card_copy);
        //}
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut cards: Vec<Card> = Vec::new();
    for item in splitted_input {
        let text = item.replace("   ", " ").replace("  ", " ");
        if let Some(card) = populate_card(&text) {
            cards.push(card);
        }
    }
    //println!("{:?}", cards);
    //println!("{:?}", cards[1]);
    let mut total_cards: Vec<Card> = Vec::new();
    get_total_cards(&cards, &cards[0], 0, &mut total_cards);
    println!("{:?}", total_cards);
    Some(total_cards.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part_two(&input);
        assert_eq!(result, Some(30));
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
