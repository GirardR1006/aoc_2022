mod shape {

    #[derive(PartialEq)]
    pub enum Shape {
        Rock,
        Paper,
        Scissor,
    }

    pub fn from_char(s: &char) -> Result<Shape, &str> {
        match s {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissor),
            _ => Err("aaaaaaah"),
        }
    }

    fn does_r_wins(l: &Shape, r: &Shape) -> bool {
        match (l, r) {
            (Shape::Rock, Shape::Paper) => true,
            (Shape::Paper, Shape::Scissor) => true,
            (Shape::Scissor, Shape::Rock) => true,
            _ => false,
        }
    }

    pub fn win_score(l: &Shape, r: &Shape) -> usize {
        let win_score = if does_r_wins(&l, &r) {
            6
        } else {
            if l == r {
                3
            } else {
                0
            }
        };
        let score = match r {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        };
        score + win_score
    }
}
pub fn solve_day(content: String) {
    let mut score = 0;
    for line in content.lines() {
        // Only one char so we know where to split
        let (l, r) = (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
        score += shape::win_score(
            &shape::from_char(&l).unwrap(),
            &shape::from_char(&r).unwrap(),
        );
    }
    println!("[Day 2] Score: {score}");
}
