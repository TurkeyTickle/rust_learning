
pub fn print_solution() {
    let data = include_str!("data/day_1.txt");
    let cals = parse(&data);

    println!("Day 1 - Part 1: {}", part_1(&cals));
    println!("Day 1 - Part 2: {}", part_2(&cals));
}

fn parse(data: &str) -> Vec<u32> {
    let mut a: Vec<u32> = data
        .split("\n\n")
        .map(|c| c.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();

    a.sort_by(|a,b| b.cmp(a));

    return a;
}

fn part_1(cals: &Vec<u32>) -> u32 {
    cals.iter().max().copied().unwrap()
}

fn part_2(cals: &Vec<u32>) -> u32 {
    cals.iter().take(3).sum()
}