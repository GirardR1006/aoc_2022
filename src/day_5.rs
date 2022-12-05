// Execute the instruction
fn do_instr(instr: &str, yard: &mut Vec<Vec<char>>) {
    let instrs: Vec<&str> = instr.split(' ').collect();
    let amount: usize = instrs[1].parse().unwrap();
    let mut src: usize = instrs[3].parse().unwrap();
    let mut dst: usize = instrs[5].parse().unwrap();
    src -= 1;
    dst -= 1;
    // println!("Move {amount} crate from stack {src} to stack {dst}.");
    // println!("Source stack before instruction {:?}", yard[src]);
    // println!("Dest stack before instruction {:?}", yard[dst]);
    for _ in 0..amount {
        match yard[src].pop() {
            Some(b_id) => yard[dst].push(b_id),
            None => (),
        }
    }
    // println!("Source stack after instruction {:?}", yard[src]);
    // println!("Dest stack after instruction {:?}", yard[dst]);
}

/// Initialize the yard state and get the list of instructions
fn parse(content: &str) -> (Vec<Vec<char>>, &str) {
    let splt: Vec<&str> = content.split("\n\n").collect();
    let yard_str = splt[0];
    let instr_str = splt[1];
    // get number of columns by getting the last element
    // of the latest line.
    let mut yard_lines = yard_str.lines();
    let n_col = yard_lines
        .next_back()
        .unwrap()
        .chars()
        .rev()
        .find(|&x| x.is_numeric())
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    let n_chars_line = n_col * 4 - 1;
    let mut yard = vec![Vec::new(); n_col];
    // reverse the iterator to parse in the correct order,
    // then on each line,
    // move by a certain amount depending on the character we get
    // Alternatively, could use the next_by iterator
    for line in yard_lines.rev() {
        let mut idx = 0;
        let chs: Vec<char> = line.chars().collect();
        while idx < n_chars_line {
            let ch = chs[idx];
            if ch == ' ' {
                idx += 4
            } else {
                let b_id = line.chars().nth(idx + 1).unwrap();
                yard[idx / 4].push(b_id);
                idx += 4
            }
        }
    }
    (yard, instr_str)
}

pub fn solve_day_first_question(content: &str) {
    let (mut yard, instrs) = parse(content);
    for instr in instrs.lines() {
        do_instr(instr, &mut yard)
    }
    for s in yard.iter_mut() {
        print!("{}", s.pop().unwrap())
    }
}

pub fn solve_day_second_question(content: &str) {
    let (mut yard, instrs) = parse(content);
    for instr in instrs.lines() {
        do_instr(instr, &mut yard)
    }
    // Rev all piles as we parsed from top to bottom, and get top id
    for s in yard.iter_mut() {
        s.reverse();
        print!("{}", s.pop().unwrap())
    }
}
