advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
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
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_b() {
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_c() {
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_d() {
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_e() {
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_f() {
        let input = "?###???????? 3,2,1";
        let result = part_one(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_total() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = part_two(&input);
        assert_eq!(result, Some(21));
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
