
mod advent_of_code_2022;

use crate::advent_of_code_2022::{self as aoc};

fn main() {
    let elf_calorie_counts = aoc::day_1::part_1::get_elf_calorie_counts();
    println!("day 1 part 1: {}", elf_calorie_counts.iter().max().unwrap());

    let day_1_2_solution = aoc::day_1::part_2::get_top_three_elf_calorie_sum(elf_calorie_counts);
    println!("day 1 part 2: {}", day_1_2_solution);

    let day_2_solution = aoc::day_2::get_solution();
    println!("day 2: {}", day_2_solution);

    let day_3_solution = aoc::day_3::get_solution();
    println!("day 3: {}", day_3_solution);
}
