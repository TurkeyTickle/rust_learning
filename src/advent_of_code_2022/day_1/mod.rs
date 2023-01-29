mod part_1;
mod part_2;

pub fn print_solution() {
    let elf_calorie_counts = part_1::get_elf_calorie_counts();
    println!("day 1 part 1: {}", elf_calorie_counts.iter().max().unwrap());

    let day_1_2_solution = part_2::get_top_three_elf_calorie_sum(elf_calorie_counts);
    println!("day 1 part 2: {}", day_1_2_solution);
}