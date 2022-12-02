use std::fs;
mod day_1;

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");
    day_1::solve_day(content);
}
