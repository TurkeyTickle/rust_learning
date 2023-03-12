pub fn print_solution() {
    let raw_data = include_str!("data/day_4.txt");
    let data = parse(&raw_data);

    println!("Day 4 - Part 1: {}", part_1(&data));
    println!("Day 4 - Part 2: {}", part_2(&data));
}

fn parse(data: &str) -> Vec<((u32, u32), (u32, u32))> {
    data.lines().map(|line| {
        let parts = line.split_once(',').unwrap();
        let a = parts.0.split_once('-').unwrap();
        let b = parts.1.split_once('-').unwrap();
        ((a.0.parse().unwrap(), a.1.parse().unwrap()), (b.0.parse().unwrap(), b.1.parse().unwrap()))
    }).collect()
}

fn part_1(data: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut result = 0;

    for set in data {
        let a: Vec<u32> = (set.0.0..=set.0.1).collect();
        let b: Vec<u32> = (set.1.0..=set.1.1).collect();
        if a.iter().all(|x| b.contains(x)) || b.iter().all(|x| a.contains(x)) {
            result += 1;
        }
    }

    result
}

fn part_2(data: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_DATA),
            [((2,4),(6,8)),
            ((2,3),(4,5)),
            ((5,7),(7,9)),
            ((2,8),(3,7)),
            ((6,6),(4,6)),
            ((2,6),(4,8))]
        );
    }

    #[test]
    fn test_part_1() {
        let data = parse(TEST_DATA);
        assert_eq!(part_1(&data), 2)
    }

    #[test]
    fn test_part_2() {
        let data = parse(TEST_DATA);
        assert_eq!(part_2(&data), 70)
    }
}