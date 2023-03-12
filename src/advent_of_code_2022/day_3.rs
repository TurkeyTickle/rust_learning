use std::collections::HashMap;

pub fn print_solution() {
    let raw_data = include_str!("data/day_3.txt");
    let data = parse(&raw_data);

    println!("Day 3 - Part 1: {}", part_1(&data));
    println!("Day 3 - Part 2: {}", part_2(&data));
}

fn parse(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

fn part_1(data: &Vec<&str>) -> u32 {
    let letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority_map: HashMap<char, u32> = HashMap::default();

    let mut index = 1;
    for letter in letters.chars() {
        priority_map.insert(letter, index);
        index += 1;
    }

    let mut result = 0;
    for set in data {
        let parts = set.split_at(set.len() / 2);

        for letter in letters.chars() {
            if parts.0.contains(letter) && parts.1.contains(letter) {
                result += priority_map.get(&letter).unwrap();
            }
        }
    }

    result
}

fn part_2(data: &Vec<&str>) -> u32 {
    let letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priority_map: HashMap<char, u32> = HashMap::default();

    let mut index = 1;
    for letter in letters.chars() {
        priority_map.insert(letter, index);
        index += 1;
    }

    let mut result = 0;
    for set in data.chunks(3) {
        for letter in letters.chars() {
            if set.iter().all(|line| line.contains(letter)) {
                result += priority_map.get(&letter).unwrap();
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_DATA),
            ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"]
        );
    }

    #[test]
    fn test_part_1() {
        let data = parse(TEST_DATA);
        assert_eq!(part_1(&data), 157)
    }

    #[test]
    fn test_part_2() {
        let data = parse(TEST_DATA);
        assert_eq!(part_2(&data), 70)
    }
}