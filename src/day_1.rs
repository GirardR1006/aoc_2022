fn rotate(maxes: &mut [i32; 3], loc: i32) {
    if loc > maxes[2] {
        if loc > maxes[1] {
            if loc > maxes[0] {
                maxes[2] = maxes[1];
                maxes[1] = maxes[0];
                maxes[0] = loc;
            } else {
                maxes[2] = maxes[1];
                maxes[1] = loc;
            }
        } else {
            maxes[2] = loc;
        }
    }
}

pub fn solve_day(content: String){
    // first element: big food
    let mut maxes = [0; 3];
    let mut loc = 0;
    for line in content.lines() {
        let res: i32 = line.parse().unwrap_or(-1);
        if res == -1 {
            rotate(&mut maxes, loc);
            loc = 0;
        } else {
            loc = loc + res;
        }
    }
    //last one for EOF
    rotate(&mut maxes, loc);
    let res: i32 = maxes.iter().sum();
    println!("Maximum amount of food : {res}")

}
