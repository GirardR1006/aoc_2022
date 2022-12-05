// Execute the instruction
fn do_instr(instr: &str, yard: &mut Vec<Vec<char>>) {
    let instrs: Vec<&str> = instr.split(' ').collect();
    let amount: usize = instrs[1].parse().unwrap();
    let src: usize = instrs[3].parse().unwrap();
    let dst: usize = instrs[5].parse().unwrap();
    for _ in 0..amount {
        let b_id = yard[src].pop().unwrap();
        yard[dst].push(b_id);
    }
}

/// Initialize the yard state and get the list of instructions
fn parse(content: &str) -> (Vec<Vec<char>>, &str) {
    let splt: Vec<&str> = content.split("\n\n").collect();
    let yard_str = splt[0];
    let instr_str = splt[1];
    // get amount of columns and consume only one element
    // of the iterator after reverse. Number of chars: n_cols * 4 - 1
    let mut yard_lines = yard_str.lines().rev();
    let n_col: usize = yard_lines
        .next()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .parse()
        .unwrap();
    let n_chars_line = n_col * 4 - 1;
    let mut yard = vec![Vec::new(); n_col];
    // re-reverse the iterator, then on each line,
    // move by a certain amount depending on the character we get
    // Alternatively, could use the advance_by nightly feature
    for line in yard_lines.rev() {
        let mut idx = 0;
        let chs : Vec<char> = line.chars().collect();
        while idx < n_chars_line{
            let ch = chs[idx];
            if  ch == ' '{
                idx += 4
            }
            else{
                let b_id = line.chars().nth(idx).unwrap();
                yard[idx/4].push(b_id);
                idx += 4
            }
        }
    }
    (yard, instr_str)
}

pub fn solve_day_first_question(content : &str){
    let (mut yard, instrs) = parse(content);
    for instr in instrs.lines(){
        do_instr(instr, &mut yard)
    }
    // Rev all piles as we parsed from top to bottom, and get top id
    for s in yard.iter_mut(){
        s.reverse();
        println!("{}",s.pop().unwrap())
    }
}

pub fn solve_day_second_question(content : &str){
    let (mut yard, instrs) = parse(content);
    for instr in instrs.lines(){
        do_instr(instr, &mut yard)
    }
    // Rev all piles as we parsed from top to bottom, and get top id
    for s in yard.iter_mut(){
        s.reverse();
        println!("{}",s.pop().unwrap())
    }
}
