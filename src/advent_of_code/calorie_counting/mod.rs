use std::str::Lines;

static TEXT: &'static str = include_str!("calories.txt");

pub fn main() {
    let mut lines: Lines = TEXT.lines();

    let elf_splits = split_elves(&mut lines);
    let elf_sums = sum_cals(elf_splits);

    let result = elf_sums.iter().max();
    println!("{}", result.unwrap());
}

fn split_elves(lines: &mut Lines) -> Vec<Vec<u32>> {
    let mut elf_splits: Vec<Vec<u32>> = Vec::new();
    elf_splits.push(Vec::new());
    let mut elf_index = 0;

    for line in lines {
        if line == "" {
            elf_index += 1;
            elf_splits.push(Vec::new());
            continue;
        }

        elf_splits[elf_index].push(line.parse().unwrap());
    }

    elf_splits
}

fn sum_cals(elf_splits: Vec<Vec<u32>>) -> Vec<u32> {
    let mut elf_sums: Vec<u32> = Vec::new();

    for elf_split in elf_splits {
        elf_sums.push(elf_split.iter().sum());
    }

    elf_sums
}


    