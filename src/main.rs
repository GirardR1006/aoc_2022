use std::fs;
mod day_1;
mod day_2;

fn main() {
    let content_day_1 = fs::read_to_string("inputs/input_day_1").unwrap();
    day_1::solve_day(content_day_1);
    let content_day_2 = fs::read_to_string("inputs/input_day_2").unwrap();
    day_2::solve_day(content_day_2);
}
