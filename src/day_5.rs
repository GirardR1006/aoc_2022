use std::num::ParseIntError;

// Execute the instruction. If all is true, preserve the
// order on the origin stack
fn move_from_to_by(yard: &mut Vec<Vec<char>>, from: usize, to: usize, by: usize, all: bool) {
    if !all {
        for _ in 0..by {
            match yard[from].pop() {
                Some(b_id) => yard[to].push(b_id),
                None => (),
            }
        }
    } else {
        let mut buf = Vec::new();
        for _ in 0..by {
            match yard[from].pop() {
                Some(b_id) => buf.push(b_id),
                None => (),
            }
        }
        for _ in 0..by {
            yard[to].push(buf.pop().unwrap())
        }
    }
}
fn do_instr<'a>(instr: &'a str, yard: &mut Vec<Vec<char>>, all: bool) -> Result<(), ParseIntError> {
    let instrs: Vec<&str> = instr.split(' ').collect();
    let amount: usize = instrs[1].parse()?;
    let mut src: usize = instrs[3].parse()?;
    let mut dst: usize = instrs[5].parse()?;
    src -= 1;
    dst -= 1;
    Ok(move_from_to_by(yard, src, dst, amount, all))
}

/// Initialize the yard state and get the list of instructions
fn parse(content: &str) -> Option<(Vec<Vec<char>>, &str)> {
    let splt: Vec<&str> = content.split("\n\n").collect();
    let yard_str = splt[0];
    let instr_str = splt[1];
    // get number of columns by getting the last element
    // of the latest line.
    let mut yard_lines = yard_str.lines();
    let n_col = yard_lines
        .next_back()?
        .chars()
        .rev()
        .find(|&x| x.is_numeric())?
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
                match line.chars().nth(idx + 1) {
                    Some(b_id) => yard[idx / 4].push(b_id),
                    None => (),
                };
                idx += 4
            }
        }
    }
    Some((yard, instr_str))
}

pub fn solve_day_first_question(content: &str) {
    let (mut yard, instrs) = parse(content).unwrap();
    for instr in instrs.lines() {
        do_instr(instr, &mut yard, false).unwrap()
    }
    print!("[Day 3] Final top crates: ");
    for s in yard.iter_mut() {
        print!("{}", s.pop().unwrap())
    }
    println!("")
}

pub fn solve_day_second_question(content: &str) {
    let (mut yard, instrs) = parse(content).unwrap();
    for instr in instrs.lines() {
        do_instr(instr, &mut yard, true).unwrap()
    }
    print!("[Day 3] Final top crates corrected: ");
    for s in yard.iter_mut() {
        print!("{}", s.pop().unwrap())
    }
    println!("")
}
