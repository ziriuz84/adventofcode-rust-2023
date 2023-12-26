advent_of_code::solution!(11);

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn prepare_grid(input: &str) -> Vec<Vec<char>> {
    //println!("{}", input);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for text in input.lines() {
        let row: Vec<char> = text.chars().collect();
        grid.push(row);
    }
    //println!("{:?}", grid);
    grid
}

fn double_rows_without_char(grid: Vec<Vec<char>>, ch: char, n_rows: i64) -> Vec<Vec<char>> {
    //println!("{:?}", grid);
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for row in grid {
        if !row.contains(&ch) {
            for i in 0..n_rows {
                new_grid.push(row.clone());
            }
        }
        new_grid.push(row.clone());
    }
    //println!("{:?}", new_grid);
    new_grid
}

fn rotate_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let number_rows = grid.len();
    let number_cols = grid[0].len();
    let mut new_grid: Vec<Vec<char>> = vec![vec![' '; number_rows]; number_cols];
    //println!("{} {}", number_rows, number_cols);
    for i in 0..number_rows {
        for j in 0..number_cols {
            new_grid[j][number_rows - 1 - i] = grid[i][j];
        }
    }
    //println!("{:?}", new_grid);
    new_grid
}

fn double_columns_without_char(grid: Vec<Vec<char>>, ch: char, n_rows: i64) -> Vec<Vec<char>> {
    //println!("{:?}", grid);
    let temp_grid: Vec<Vec<char>> = rotate_grid(grid);
    let number_rows = temp_grid.len();
    let number_cols = temp_grid[0].len();
    //println!("{} {}", number_rows, number_cols);
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for row in temp_grid {
        if !row.contains(&ch) {
            for i in 0..n_rows {
                new_grid.push(row.clone());
            }
        }
        new_grid.push(row.clone());
    }
    //println!("{:?}", new_grid);
    new_grid
}

fn search_char(input: Vec<Vec<char>>, ch: char) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for i in 0..input.len() {
        let row = input[i].clone();
        for j in 0..row.len() {
            if row[j] == ch {
                let point = Point {
                    x: i as i64,
                    y: j as i64,
                };
                result.push(point);
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = prepare_grid(input);
    let grid_rows_doubled = double_rows_without_char(grid, '#', 1);
    let final_grid = double_columns_without_char(grid_rows_doubled, '#', 1);
    let mut sum: i64 = 0;
    let points: Vec<Point> = search_char(final_grid, '#');
    let temp_points = points.clone();
    for point in points {
        let new_points = temp_points.clone();
        for new_point in new_points {
            let dx = new_point.x - point.x;
            let dy = new_point.y - point.y;
            sum += dx.abs() + dy.abs();
            //println!("{:?} -> {:?}", point, new_point);
            //println!("{} {} {}", dx, dy, sum);
        }
    }
    Some(sum / 2)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = prepare_grid(input);
    let grid_rows_doubled = double_rows_without_char(grid, '#', 10);
    let final_grid = double_columns_without_char(grid_rows_doubled, '#', 10);
    let mut sum: i64 = 0;
    let points: Vec<Point> = search_char(final_grid, '#');
    let temp_points = points.clone();
    println!("{}", points.len());
    for point in points {
        let new_points = temp_points.clone();
        for new_point in new_points {
            let dx = new_point.x - point.x;
            let dy = new_point.y - point.y;
            sum += dx.abs() + dy.abs();
        }
    }
    Some(sum / 2)
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
    fn test_two_a() {
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
        let result = part_two(&input);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_two_b() {
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
        let result = part_two(&input);
        assert_eq!(result, Some(8410));
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
