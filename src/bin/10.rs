advent_of_code::solution!(10);

use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vect2D {
    x: i32,
    y: i32,
}

lazy_static! {
    static ref N: Vect2D = Vect2D::new(0, -1);
    static ref S: Vect2D = Vect2D::new(0, 1);
    static ref W: Vect2D = Vect2D::new(-1, 0);
    static ref E: Vect2D = Vect2D::new(1, 0);
    static ref EMPTY: Vec<Vect2D> = vec![];
    static ref TILES: HashMap<char, Vec<Vect2D>> = HashMap::from([
        ('|', vec![*N, *S]),
        ('-', vec![*E, *W]),
        ('L', vec![*N, *E]),
        ('J', vec![*N, *W]),
        ('7', vec![*S, *W]),
        ('F', vec![*S, *E])
    ]);
}

impl Vect2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug)]
struct Map<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Map<'a> {
    fn get_connectors(&self, pos: &Vect2D) -> &Vec<Vect2D> {
        if pos.y < 0 || pos.y >= self.lines.len() as i32 {
            return &EMPTY;
        }
        let row = self.lines.get(pos.y as usize).unwrap();
        if pos.x < 0 || pos.x >= row.len() as i32 {
            return &EMPTY;
        }
        let character = row.as_bytes()[pos.x as usize] as char;
        TILES.get(&character).unwrap_or(&EMPTY)
    }
}

fn collect_path(map: &Map) -> HashSet<Vect2D> {
    let start = map
        .lines
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.find('S').map(|x| Vect2D::new(x as i32, y as i32)))
        .expect("Could not find start");
    let mut path: HashSet<Vect2D> = HashSet::new();
    path.insert(start);
    let directions: Vec<Vect2D> = vec![*N, *S, *W, *E];
    let mut pos: Vect2D = start;
    let step = directions
        .iter()
        .find(|dir| {
            let connectors = map.get_connectors(&pos.add(dir));
            connectors
                .iter()
                .any(|conn| conn.x == -dir.x && conn.y == -dir.y)
        })
        .expect("no first step found");
    pos = pos.add(step);
    path.insert(pos);
    while !path.len() > 1 && pos != start {
        let connectors = map.get_connectors(&pos);
        pos = connectors
            .iter()
            .map(|dir| pos.add(dir))
            .find(|next_pos| !path.contains(next_pos) || (next_pos == &start && path.len() > 2))
            .unwrap_or_else(|| panic!("no next step found {:?}", pos));
        path.insert(pos);
    }
    path
}

fn parse_input(input: &str) -> Map {
    Map {
        lines: input.lines().collect(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = parse_input(&input);
    println!("{:?}", map);
    let path: HashSet<Vect2D> = collect_path(&map);
    Some(path.len() as u32 / 2)
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
    fn test_part_two_a() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part_two(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_b() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let result = part_two(&input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_c() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let result = part_two(&input);
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
