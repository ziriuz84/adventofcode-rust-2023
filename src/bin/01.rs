advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for text in splitted_input {
        let mut numbers = Vec::new();
        let char_list: Vec<char> = text.chars().collect();
        for c in char_list {
            if c.is_digit(10) {
                numbers.push(c);
            }
        }
        //let string: String = format!("{}{}", numbers[0], numbers.last().unwrap());
        //sum += string.parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for text in splitted_input {
        let mut numbers = Vec::new();
        let mut positions = Vec::new();
        let mut new_text = text.to_string();
        new_text = new_text
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        //.chars()
        //.filter(|c| c.is_digit(10))
        //.collect();
        println!("{}", text);
        println!("{}", new_text);
        let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for (i, c) in new_text.chars().enumerate() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10).unwrap());
                positions.push(i);
            }
        }
        println!("{:?} {:?}", numbers, positions);
        let mut string = String::new();
        if numbers.len() == 1 {
            string = format!("{}", numbers[0]);
        } else if numbers.len() == 0 {
            println!("posizione vuota");
        } else {
            string = format!("{}{}", numbers[0], numbers.last().unwrap());
        }
        println!("{}", string.parse::<u32>().unwrap());
        sum += string.parse::<u32>().unwrap();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part_two(&input);
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_part_two_example() {
        let input = "46threevqs8114
threetwoonez1gtrd
6ffxbtff
769twotwo6rv9
gjrcjrkvghthreegqqrg82qbct
zkxjhgprtrlcfeight795five8
99seven3vdcgvmvxtjtwodc5
three5eightthree3four3vtkkqrgxs
four863mrrnrsxrkone
sevenntgvnrrqfvxh2ttnkgffour8fiveone
49fbsfb
3rbmlsksg
ztgszqjjsrtmgqx6572
3bqnfxkdbonesixseven
mfgx32ftpbhgngm7
fzrpfhbfvj6dbxbtfs7twofksfbshrzkdeightwoqg
2xcftwo
cshmmltsml4fiveeightdn
eightthsix1
two1gvfxcqnrfnbeightthreexznhbmmk
3917sevenvxqxntcgxskh
ksctcnfxdsk96drlbjkthreesfqlvnpvfbcbmg
8nvrmzfs46
5dbgltmgg1xvtqfkdxsrxzltwo1pgqlqndlc
eight7four6rpbtmjzj5
41seven
gpmfhninexxgqr6
15sixdrhxzcmqf
8fivettgmcslxptwofivelckzvfkl
67ninetjngsrvcpxb8eighteightwofh
4nine7oneighthm
njtwonefvhjplkjgvsevenbjg77
eighthrspkszngkpdtzdpcsmjnvlnhcm9pqmpkxqmbtmbv
ninehthhgbfsrrbpn2qpcflhgdvh9twotpzkvzmmsj
6fourzpjthmkrkvqkvvp
vnrnkfp6
pfouronefour6
87fourmznhvmt7
nxbssjc1sevenvrcjlczct6ninekclbffs
eightrpzsdggsixthree9dhrnqtjcbxthree9";
        let result = part_two(&input);
        assert_eq!(result, Some(2142));
    }

    #[test]
    fn test_part_two_singlw() {
        let input = "46threevqs8114";
        let result = part_two(&input);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
