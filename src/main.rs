use std::fs;
mod day_1;
mod day_2;
mod day_5;

fn main() {
    let content_day_1 = fs::read_to_string("inputs/input_day_1").unwrap();
    day_1::solve_day(content_day_1);
    let content_day_2 = fs::read_to_string("inputs/input_day_2").unwrap();
    day_2::solve_day_first_question(&content_day_2);
    day_2::solve_day_second_question(&content_day_2);
    let content_day_5 = fs::read_to_string("inputs/input_day_5").unwrap();
    day_5::solve_day_first_question(&content_day_5);
    day_5::solve_day_second_question(&content_day_5);
}
