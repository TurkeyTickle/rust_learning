use std::collections::HashMap;


pub fn print_solution() {
    let raw_data = include_str!("data/day_2.txt");
    let data = parse(&raw_data);

    println!("Day 2 - Part 1: {}", part_1(&data));
    println!("Day 2 - Part 2: {}", part_2(&data));
}

fn parse(data: &str) -> Vec<(char, char)> {
    data.lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let first_char = parts.next().unwrap().parse::<char>().unwrap();
            let second_char = parts.next().unwrap().parse::<char>().unwrap();
            (first_char, second_char)
        })
        .collect()
}

fn part_1(data: &Vec<(char, char)>) -> u32 {
    let opponent_move_map = HashMap::from([
        ('A', 'R'),
        ('B', 'P'),
        ('C', 'S'),
    ]);

    let self_move_map = HashMap::from([
        ('X', 'R'),
        ('Y', 'P'),
        ('Z', 'S'),
    ]);

    let winning_move_map = HashMap::from([
        ('R', 'P'),
        ('P', 'S'),
        ('S', 'R'),
    ]);
    
    let losing_move_map = HashMap::from([
        ('R', 'S'),
        ('P', 'R'),
        ('S', 'P'),
    ]);

    let move_scores = HashMap::from([
        ('R', 1),
        ('P', 2),
        ('S', 3),
    ]);

    let outcome_scores = HashMap::from([
        ('L', 0),
        ('D', 3),
        ('W', 6),
    ]);

    let mut score = 0;

    for m in data {
        let opponent_move = opponent_move_map.get(&m.0).unwrap();
        let self_move = self_move_map.get(&m.1).unwrap();

        let win = winning_move_map.get(&opponent_move).unwrap() == self_move;
        let lose = losing_move_map.get(&opponent_move).unwrap() == self_move;

        score += move_scores.get(&self_move).unwrap();
        score += outcome_scores.get(&(if win { 'W' } else { if lose { 'L' } else { 'D' }})).unwrap();
    }

    score
}

fn part_2(data: &Vec<(char, char)>) -> u32 {
    let opponent_move_map = HashMap::from([
        ('A', 'R'),
        ('B', 'P'),
        ('C', 'S'),
    ]);

    let desired_outcome_map = HashMap::from([
        ('X', 'L'),
        ('Y', 'D'),
        ('Z', 'W'),
    ]);

    let winning_move_map = HashMap::from([
        ('R', 'P'),
        ('P', 'S'),
        ('S', 'R'),
    ]);
    
    let losing_move_map = HashMap::from([
        ('R', 'S'),
        ('P', 'R'),
        ('S', 'P'),
    ]);

    let move_scores = HashMap::from([
        ('R', 1),
        ('P', 2),
        ('S', 3),
    ]);

    let outcome_scores = HashMap::from([
        ('L', 0),
        ('D', 3),
        ('W', 6),
    ]);

    let mut score = 0;

    for m in data {
        let opponent_move = opponent_move_map.get(&m.0).unwrap();
        let desired_outcome = desired_outcome_map.get(&m.1).unwrap();

        let self_move = match desired_outcome {
            'W' => winning_move_map.get(&opponent_move).unwrap(),
            'L' => losing_move_map.get(&opponent_move).unwrap(),
            _ => opponent_move
        };

        score += move_scores.get(&self_move).unwrap();
        score += outcome_scores.get(&desired_outcome).unwrap();
    }

    score
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(TEST_DATA),
            [('A', 'Y'), ('B', 'X'), ('C', 'Z')]
        );
    }

    #[test]
    fn test_part_1() {
        let data = parse(TEST_DATA);
        assert_eq!(part_1(&data), 15)
    }

    #[test]
    fn test_part_2() {
        let data = parse(TEST_DATA);
        assert_eq!(part_2(&data), 12)
    }
}