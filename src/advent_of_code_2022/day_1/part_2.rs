pub fn get_top_three_elf_calorie_sum(mut elf_calorie_counts: Vec<u32>) -> u32 {
    elf_calorie_counts.sort();
    elf_calorie_counts.reverse();
    elf_calorie_counts[..3].iter().sum()
}