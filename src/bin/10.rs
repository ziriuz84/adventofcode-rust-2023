use grid::*;
advent_of_code::solution!(10);

fn parse_input(input: &str) -> Grid<char> {
    let start_value: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let col_size = start_value[0].len();
    Grid::from_vec(start_value.into_iter().flatten().collect(), col_size)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    println!("{:?}", grid);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let result = part_one(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_b() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let result = part_one(&input);
        assert_eq!(result, Some(8));
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
