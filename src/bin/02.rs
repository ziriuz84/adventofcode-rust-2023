use std::borrow::BorrowMut;

advent_of_code::solution!(2);

#[derive(Debug)]
pub struct Color {
    green: u32,
    red: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    // TODO
    id: u32,
    colors: Color,
    power: u32,
}

pub fn control_fields(fields: Vec<&str>) -> bool {
    let mut result: bool = true;
    for item in fields {
        let field: Vec<&str> = item[1..].split(' ').collect();
        //println!("{} {}", field[1], field[0]);
        if field[1] == "red" && field[0].parse::<u32>().unwrap() > 12 {
            result = false;
        }
        if field[1] == "green" && field[0].parse::<u32>().unwrap() > 13 {
            result = false;
        }
        if field[1] == "blue" && field[0].parse::<u32>().unwrap() > 14 {
            result = false;
        }
    }
    result
}

pub fn control_balls(game: Vec<&str>) -> bool {
    let mut result: bool = false;
    for item in game {
        let fields: Vec<&str> = item.split(',').collect();
        result = control_fields(fields);
        if !result {
            break;
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    for text in splitted_input {
        let splitted_text: Vec<&str> = text.split(':').collect();
        let games: Vec<&str> = splitted_text[1].split(';').collect();
        let game_id_text: Vec<&str> = splitted_text[0].split(' ').collect();
        let game_id = game_id_text[1];
        if control_balls(games) {
            sum += game_id.parse::<u32>().unwrap();
        }
    }
    Some(sum)
}

pub fn control_max_color(colors: Vec<&str>, mut color_ref: Color) -> Color {
    for item in colors {
        let field: Vec<&str> = item[1..].split(' ').collect();
        //println!("{} {}", field[1], field[0]);
        if field[1] == "red" && field[0].parse::<u32>().unwrap() > color_ref.red {
            color_ref.red = field[0].parse::<u32>().unwrap();
        }
        if field[1] == "green" && field[0].parse::<u32>().unwrap() > color_ref.green {
            color_ref.green = field[0].parse::<u32>().unwrap();
        }
        if field[1] == "blue" && field[0].parse::<u32>().unwrap() > color_ref.blue {
            color_ref.blue = field[0].parse::<u32>().unwrap();
        }
    }
    color_ref
}

pub fn count_balls(input: Vec<&str>, mut color: Color) -> Color {
    for game in input {
        let balls: Vec<&str> = game.split(',').collect();
        color = control_max_color(balls, color);
    }
    color
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    for text in splitted_input {
        let splitted_text: Vec<&str> = text.split(':').collect();
        let games: Vec<&str> = splitted_text[1].split(';').collect();
        let game_id_text: Vec<&str> = splitted_text[0].split(' ').collect();
        let mut game = Game {
            id: game_id_text[1].parse::<u32>().unwrap(),
            colors: Color {
                red: 0,
                green: 0,
                blue: 0,
            },
            power: 1,
        };
        let colors = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        let max_balls = count_balls(games, colors);
        game.colors = max_balls;
        game.power = game.colors.green * game.colors.red * game.colors.blue;
        sum += game.power;
    }
    Some(sum)
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
