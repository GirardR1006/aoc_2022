use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut max = 0;
    let mut loc = 0;
    for line in content.lines() {
        let res: i32 = line.parse().unwrap_or(-1);
        if res == -1 {
            if loc > max {
                max = loc;
            }
            loc = 0;
        } else {
            loc = loc + res;
        }
    }
    println!("Maximum amount of food : {max}")
}
