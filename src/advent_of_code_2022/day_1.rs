pub fn print_solution() {
    let raw_data = include_str!("data/day_1.txt");
    let mut data = parse(&raw_data);

    println!("Day 1 - Part 1: {}", part_1(&data));
    println!("Day 1 - Part 2: {}", part_2(&mut data));
}

fn parse(data: &str) -> Vec<u32> {
    data.split("\n\n")
        .map(|c| c.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect()
}

fn part_1(data: &Vec<u32>) -> u32 {
    data.iter().max().copied().unwrap()
}

fn part_2(data: &mut Vec<u32>) -> u32 {
    data.sort_by(|a, b| b.cmp(a));
    data.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_DATA),
            [6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn test_part_1() {
        let data = parse(TEST_DATA);
        assert_eq!(part_1(&data), 24000)
    }

    #[test]
    fn test_part_2() {
        let mut data = parse(TEST_DATA);
        assert_eq!(part_2(&mut data), 45000)
    }
}
