
mod advent_of_code_2022;

use crate::advent_of_code_2022::{self as aoc};

fn main() {
    let day_1_solution = aoc::day_1::get_solution();
    println!("day 1: {}", day_1_solution);

    let day_2_solution = aoc::day_2::get_solution();
    println!("day 2: {}", day_2_solution);

}
