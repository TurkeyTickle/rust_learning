
pub fn print_solution() {
    let raw_data = include_str!("data/day_5.txt");
    let data = parse(&raw_data);

    println!("Day 5 - Part 1: {}", part_1(&data));
    println!("Day 5 - Part 2: {}", part_2(&data));
}

fn parse(data: &str) -> (Vec<Crate>, Vec<(u32, u32, u32 )>) {
    let parts = data.split_once("\n\n").unwrap();

    let mut crates: Vec<Crate> = vec![];
    let index_row = parts.0.lines().last().unwrap();

    let mut x: usize = 0;
    for c in index_row.chars() {
        if !char::is_whitespace(c) {
            let mut y = 0;
            for line in parts.0.lines().rev().skip(1) {
                let name = line.chars().collect::<Vec<char>>()[x];
                if !char::is_whitespace(name) {
                    crates.push(Crate { x: x / 4, y, name });
                }
                y += 1;
            }
        }

        x += 1;
    }

    let mut instructions: Vec<(_, _, _)> = vec![];
    for line in parts.1.lines() {
        let mut matches = line.matches(char::is_numeric);
        instructions.push((matches.next().unwrap().parse().unwrap(), matches.next().unwrap().parse().unwrap(), matches.next().unwrap().parse().unwrap()));
    }

    (crates, instructions)
}

fn part_1(data: &(Vec<Crate>, Vec<(u32, u32, u32)>)) -> &str {
    ""
}

fn part_2(data: &(Vec<Crate>, Vec<(u32, u32, u32)>)) -> &str {
    ""
}

#[derive(Debug)]
struct Crate {
    name: char,
    x: usize,
    y: usize,
}

impl PartialEq for Crate {
    fn eq(&self, other: &Crate) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.name.eq(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_parse() {
        let result = parse(TEST_DATA);
        assert_eq!(result.0, vec![
            Crate {x: 0, y: 0, name: 'Z' },
            Crate {x: 0, y: 1, name: 'N' },
            Crate {x: 1, y: 0, name: 'M' },
            Crate {x: 1, y: 1, name: 'C' },
            Crate {x: 1, y: 2, name: 'D' },
            Crate {x: 2, y: 0, name: 'P' },
        ]);
        assert_eq!(result.1, vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2)
        ]);
    }

    #[test]
    fn test_part_1() {
        let data = parse(TEST_DATA);
        assert_eq!(part_1(&data), "CMZ")
    }

    #[test]
    fn test_part_2() {
        let data = parse(TEST_DATA);
        assert_eq!(part_2(&data), "")
    }
}
