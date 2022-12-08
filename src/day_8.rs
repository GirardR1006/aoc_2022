use ndarray::{self, Array, Array2, ArrayViewMut1, Axis};

fn parse(content: &str) -> Array2<(i32, bool)> {
    let mut vv = Vec::new();
    let mut n_col = 0;
    let mut n_row = 0;
    for (ir, line) in content.lines().enumerate() {
        for (ic, c) in line.chars().enumerate() {
            vv.push((c.to_digit(10).unwrap() as i32, false));
            n_col = ic+1;
        }
        n_row = ir+1;
    }
    let trees = Array::from_shape_vec((n_row, n_col), vv).unwrap();
    trees
}

// set if trees are visible either on a row or a column
fn set_visible(vec: &mut ArrayViewMut1<(i32, bool)>) {
    vec.into_iter().fold(-1, |acc, x: &mut (i32, bool)| {
        if x.0 > acc {
            x.1 = true;
            x.0
        } else {
            acc
        }
    });
}

pub fn solve_day_first_question(content: &str) {
    let mut trees = parse(content);
    for mut row in trees.axis_iter_mut(Axis(0)) {
        set_visible(&mut row);
        row.invert_axis(Axis(0));
        set_visible(&mut row);
        row.invert_axis(Axis(0));
    }
    for mut col in trees.axis_iter_mut(Axis(1)) {
        set_visible(&mut col);
        col.invert_axis(Axis(0));
        set_visible(&mut col);
        col.invert_axis(Axis(0));
    }
    println!(
        "[Day 8] Number of visible trees: {}",
        trees.fold(0, |acc, x| if x.1 { acc + 1 } else { acc })
    )
}
