use grid::*;
advent_of_code::solution!(11);

fn prepare_grid(input: &str) -> Grid<char> {
    let mut grid: Grid<char> = Grid::new(140, 140);
    for text in input.lines() {
        let row: Vec<char> = text.chars().collect();
        grid.push_row(row);
    }
    grid
}

fn double_empty_rows(grid: Grid<char>) -> Grid<char> {
    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = prepare_grid(input);
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
    fn test_one() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = part_one(&input);
        assert_eq!(result, Some(374));
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
