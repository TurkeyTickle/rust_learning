mod part_1;
mod part_2;

pub fn print_solution() {
    let self_total_score = part_1::get_self_total_score();
    println!("day 2 part 1: {}", self_total_score); 

    let self_total_score_2 = part_2::get_self_total_score();
    println!("day 2 part 2: {}", self_total_score_2);
}