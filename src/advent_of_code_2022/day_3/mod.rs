mod part_1;

pub fn print_solution() {
    let sums = part_1::get_dup_item_priority_sums();
    println!("day 3 part 1: {}", sums);
}