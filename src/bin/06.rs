advent_of_code::solution!(6);

#[derive(Debug)]
pub struct Runs {
    time: u32,
    distance: u32,
    races: Vec<u32>,
}

pub fn populate_runs(input: Vec<&str>) -> Vec<Runs> {
    let mut runs: Vec<Runs> = Vec::new();
    let temp_times: Vec<&str> = input[0].split(':').collect();
    let times: Vec<u32> = temp_times[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let temp_distances: Vec<&str> = input[1].split(':').collect();
    let distances: Vec<u32> = temp_distances[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    for i in 0..times.len() {
        let mut races = Vec::new();
        for j in 0..times[i] {
            races.push((times[i] - j) * j);
        }
        runs.push(Runs {
            time: times[i],
            distance: distances[i],
            races,
        })
    }
    runs
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let runs = populate_runs(splitted_input);
    let mut result = 1;
    println!("{:?}", runs);
    for run in runs {
        let mut sum = 0;
        for race in run.races {
            if race > run.distance {
                sum += 1;
            }
        }
        result *= sum;
    }
    Some(result)
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
