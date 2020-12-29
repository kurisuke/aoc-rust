use crate::day::Day;

pub struct Day02 {}

fn move_key_star1(pos: (isize, isize), dir: char) -> (isize, isize) {
    match dir {
        'L' => ((pos.0 - 1).max(0), pos.1),
        'R' => ((pos.0 + 1).min(2), pos.1),
        'D' => (pos.0, (pos.1 - 1).max(0)),
        'U' => (pos.0, (pos.1 + 1).min(2)),
        _ => {
            panic!("Invalid direction: {}", dir);
        }
    }
}

fn move_key_star2(pos: (isize, isize), dir: char) -> (isize, isize) {
    let cand = match dir {
        'L' => (pos.0 - 1, pos.1),
        'R' => (pos.0 + 1, pos.1),
        'D' => (pos.0, pos.1 - 1),
        'U' => (pos.0, pos.1 + 1),
        _ => {
            panic!("Invalid direction: {}", dir);
        }
    };
    if cand.0.abs() + cand.1.abs() > 2 {
        pos
    } else {
        cand
    }
}

fn pos_to_key_star1(pos: &(isize, isize)) -> isize {
    (2 - pos.1) * 3 + pos.0 + 1
}

fn pos_to_key_star2(pos: &(isize, isize)) -> char {
    match pos {
        (0, 2) => '1',
        (-1, 1) => '2',
        (0, 1) => '3',
        (1, 1) => '4',
        (-2, 0) => '5',
        (-1, 0) => '6',
        (0, 0) => '7',
        (1, 0) => '8',
        (2, 0) => '9',
        (-1, -1) => 'A',
        (0, -1) => 'B',
        (1, -1) => 'C',
        (0, -2) => 'D',
        _ => {
            panic!("Invalid pos: {:?}", pos);
        }
    }
}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let mut pos = (1, 1);
        let mut s = String::new();
        for line in input.lines() {
            if !line.is_empty() {
                for c in line.chars() {
                    pos = move_key_star1(pos, c);
                }
                s += &format!("{}", pos_to_key_star1(&pos));
            }
        }
        s
    }

    fn star2(&self, input: &str) -> String {
        let mut pos = (-2, 0);
        let mut s = String::new();
        for line in input.lines() {
            if !line.is_empty() {
                for c in line.chars() {
                    pos = move_key_star2(pos, c);
                }
                s.push(pos_to_key_star2(&pos));
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day02 {};
        let input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        assert_eq!(d.star1(input), "1985");
        assert_eq!(d.star2(input), "5DB3");
    }
}
