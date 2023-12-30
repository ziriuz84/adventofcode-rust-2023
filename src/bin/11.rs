use std::{
    cmp::{max, min},
    usize,
};
advent_of_code::solution!(11);

#[derive(Debug, Clone)]
struct Point {
    x: u64,
    y: u64,
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

fn double_rows_without_char(grid: Vec<Vec<char>>, ch: char, n_rows: u64) -> Vec<Vec<char>> {
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
    println!("{} {}", new_grid.len(), new_grid[0].len());
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
    println!("{} {}", new_grid.len(), new_grid[0].len());
    //println!("{:?}", new_grid);
    new_grid
}

fn double_columns_without_char(grid: Vec<Vec<char>>, ch: char, n_rows: u64) -> Vec<Vec<char>> {
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
    println!("{} {}", new_grid.len(), new_grid[0].len());
    new_grid
}

fn search_char(input: Vec<Vec<char>>, ch: char) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for i in 0..input.len() {
        let row = input[i].clone();
        for j in 0..row.len() {
            if row[j] == ch {
                let point = Point {
                    x: i as u64,
                    y: j as u64,
                };
                result.push(point);
            }
        }
    }
    result
}

fn prepare_data_points(grid: &Vec<Vec<char>>) -> Vec<Point> {
    search_char(grid.clone(), '#')
}

fn search_empty_rows(input: &Vec<Vec<char>>) -> Vec<u64> {
    let mut rows: Vec<u64> = vec![0; input.len()];
    for i in 0..input.len() {
        rows[i] = if i > 0 { rows[i - 1] } else { 0 };
        if !input[i].contains(&'#') {
            rows[i] += 1;
        }
    }
    rows
}

fn search_empty_columns(input: &Vec<Vec<char>>) -> Vec<u64> {
    let mut columns: Vec<u64> = vec![0; input[0].len()];
    for i in 0..input[0].len() {
        let mut presence = false;
        for j in 0..input.len() {
            if input[j][i] == '#' {
                presence = true;
                break;
            }
        }
        columns[i] = if i > 0 { columns[i - 1] } else { 0 };
        if !presence {
            columns[i] += 1;
        }
    }
    columns
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = prepare_grid(input);
    let mut sum: u64 = 0;
    let points: Vec<Point> = prepare_data_points(&grid);
    let rows = search_empty_rows(&grid);
    println!("{:?}", rows);
    let columns = search_empty_columns(&grid);
    println!("{:?}", columns);
    for i in 0..points.len() {
        for j in i..points.len() {
            //let dx = new_point.x - point.x;
            //let dy = new_point.y - point.y;
            let (x1, y1) = (min(points[i].x, points[j].x), min(points[i].y, points[j].y));
            let (x2, y2) = (max(points[i].x, points[j].x), max(points[i].y, points[j].y));
            let dx = x2 - x1;
            let dy = y2 - y1;
            //sum += dx.abs() + dy.abs();
            sum += dx
                + dy
                + (rows[x2 as usize] - rows[x1 as usize])
                + (columns[y2 as usize] - columns[y1 as usize]);
            //println!("{:?} -> {:?}", point, new_point);
            //println!("{} {} {}", dx, dy, sum);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = prepare_grid(input);
    let mut sum: u64 = 0;
    let points: Vec<Point> = prepare_data_points(&grid);
    let rows = search_empty_rows(&grid);
    let columns = search_empty_columns(&grid);
    for i in 0..points.len() {
        for j in i..points.len() {
            //let dx = new_point.x - point.x;
            //let dy = new_point.y - point.y;
            let (x1, y1) = (min(points[i].x, points[j].x), min(points[i].y, points[j].y));
            let (x2, y2) = (max(points[i].x, points[j].x), max(points[i].y, points[j].y));
            let dx = x2 - x1;
            let dy = y2 - y1;
            //sum += dx.abs() + dy.abs();
            sum += dx
                + dy
                + (rows[x2 as usize] - rows[x1 as usize]) * (1000000 - 1)
                + (columns[y2 as usize] - columns[y1 as usize]) * (1000000 - 1);
            //println!("{:?} -> {:?}", point, new_point);
            //println!("{} {} {}", dx, dy, sum);
        }
    }
    Some(sum)
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
