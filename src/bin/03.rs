advent_of_code::solution!(3);

#[derive(Debug)]
pub struct Number {
    value: u32,
    x_coord: u32,
    y_coord: u32,
    length: u32,
}

#[derive(Debug)]
pub struct Symbol {
    x_coord: u32,
    y_coord: u32,
    value: char,
}

pub fn populate_number_vec<'a>(vec: &'a mut Vec<Number>, row: usize, input: &str) {
    let input_text = input
        .replace('!', ",!,")
        .replace('@', ",@,")
        .replace('/', ",/,")
        .replace('|', ",|,")
        .replace('\\', ",\\,")
        .replace('$', ",$,")
        .replace('%', ",%,")
        .replace('^', ",^,")
        .replace('&', ",&,")
        .replace('*', ",*,")
        .replace('(', ",(,")
        .replace(')', ",),")
        .replace('+', ",+,")
        .replace('=', ",=,")
        .replace('-', ",-,")
        .replace('.', ",.,")
        .replace('#', ",#,");
    let mut text: Vec<&str> = input_text.split(',').collect();
    //println!("{:?}", text);
    text.retain(|s| !s.is_empty());
    let mut temp_x: u32 = 0;
    for item in text {
        if !item.is_empty() && item.parse::<u32>().is_ok() {
            let x_coord = input.find(item).unwrap() as u32;
            let number: Number = Number {
                value: item.parse::<u32>().unwrap(),
                y_coord: row as u32,
                length: item.len() as u32,
                x_coord: x_coord,
            };
            temp_x = x_coord + temp_x;
            vec.push(number);
        }
    }
}

pub fn populate_symbol_vec<'a>(vec: &'a mut Vec<Symbol>, row: usize, input: &str) {
    let input_text = input
        .replace('!', ",!,")
        .replace('@', ",@,")
        .replace('/', ",/,")
        .replace('|', ",|,")
        .replace('\\', ",\\,")
        .replace('$', ",$,")
        .replace('%', ",%,")
        .replace('^', ",^,")
        .replace('&', ",&,")
        .replace('*', ",*,")
        .replace('(', ",(,")
        .replace(')', ",),")
        .replace('+', ",+,")
        .replace('=', ",=,")
        .replace('-', ",-,")
        .replace('.', ",.,")
        .replace('#', ",#,");
    //println!("{}", input_text);
    let mut text: Vec<&str> = input_text.split(',').collect();
    text.retain(|s| !s.is_empty() && !s.contains('.'));
    for item in text {
        if !item.is_empty() && item.chars().all(|c| !c.is_alphanumeric()) {
            let symbol: Symbol = Symbol {
                y_coord: row as u32,
                x_coord: input.find(item).unwrap() as u32,
                value: item.parse::<char>().unwrap(),
            };
            println!("{:?}", symbol);
            vec.push(symbol);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut row: usize = 0;
    let mut string_len = 0;
    for text in &splitted_input {
        println!("{}", text);
        populate_number_vec(&mut numbers, row, text);
        populate_symbol_vec(&mut symbols, row, text);
        row += 1;
        string_len = text.len();
    }
    /*
    for n in &numbers {
        let mut l_limit: u32 = 0;
        if n.x_coord != 0 {
            l_limit = n.x_coord - 1;
        }
        let mut r_limit: u32 = string_len as u32;
        if n.x_coord != string_len as u32 {
            r_limit = n.x_coord + n.length;
        }
        let coords = [l_limit, r_limit];
        println!("{}", n.y_coord);
        println!("{:?}", coords);
        let full_coords = l_limit..r_limit;
        println!("{:?}", full_coords);
        for s in &symbols {
            println!("{:?}", s);
            if (s.y_coord == n.y_coord && coords.contains(&s.x_coord))
                || (s.y_coord != splitted_input.len() as u32
                    && s.y_coord + 1 == n.y_coord
                    && full_coords.contains(&s.x_coord))
                || (s.y_coord != 0
                    && s.y_coord - 1 == n.y_coord
                    && full_coords.contains(&s.x_coord))
            {
                println!("{:?}", n);
                println!("{:?}", s);
                println!("{}", n.value);
                sum += n.value;
            }
        }
    }
        */
    //println!("{:?}", numbers);
    //println!("{:?}", symbols);
    for s in &symbols {
        println!("{:?}", s);
        for n in &numbers {
            //println!("{}", n.value);
            if n.y_coord == s.y_coord {
                if n.x_coord + n.length == s.x_coord {
                    println!("Found a");
                    //println!("{:?}", n);
                    //println!("{:?}", s);
                    println!("{}", n.value);
                    sum += n.value;
                }
                if n.x_coord == s.x_coord + 1 {
                    println!("Found b");
                    //println!("{:?}", n);
                    //println!("{:?}", s);
                    println!("{}", n.value);
                    sum += n.value;
                }
            } else if (s.y_coord != splitted_input.len() as u32 && n.y_coord == s.y_coord + 1)
                || (s.y_coord != 0 && n.y_coord == s.y_coord - 1)
            {
                if n.x_coord <= s.x_coord + 1 && n.x_coord + n.length >= s.x_coord {
                    //println!("{} {}+{}", s.x_coord, n.x_coord, n.length);
                    //println!("{} {}", s.y_coord, n.y_coord);
                    println!("Found c");
                    //println!("{:?}", n);
                    //println!("{:?}", s);
                    println!("{}", n.value);
                    sum += n.value;
                }
            }
        }
    }
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
