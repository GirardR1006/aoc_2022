// Execute the instruction
fn do_instr(instr: &str, yard: &mut Vec<Vec<char>>) {
    let instrs: Vec<&str> = instr.split(' ').collect();
    let amount: usize = instrs[1].parse().unwrap();
    let src: usize = instrs[3].parse().unwrap();
    let dst: usize = instrs[5].parse().unwrap();
    for _ in 0..amount {
        yard[dst].push(yard[src].pop().unwrap());
    }
}

fn parse(content: String) {
    let splt: Vec<&str> = content.split("\n\n").collect();
    let yard_str = splt[0];
    let instr_str = splt[1];
    // get amount of columns and consume only one element
    // of the iterator after reverse
    let yard_lines = yard_str.lines().rev();
    let n_col: usize = yard_lines
        .next()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .parse()
        .unwrap();
    let mut yard = vec![Vec::new(); n_col];
    // re-reverse the iterator, then on each line, split on (' ') and add the relevant character
    for line in yard_lines.rev() {
        for (idx, ch) in line.split(' ').enumerate() {
            match ch {
                " " => (),
                c => {
                    let box_id = c.trim_matches(&['[', ']']);
                    yard[idx].push(box_id);
                }
            }
        }
    }
}
