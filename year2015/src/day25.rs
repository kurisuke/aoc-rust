use common::day::Day;

pub struct Day25 {}

fn index_of(row: usize, col: usize) -> usize {
    let x = (1..=row).sum::<usize>();
    let y = (row + 2..=row + 1 + col).sum::<usize>();
    x + y
}

fn parse_input(input: &str) -> (usize, usize) {
    let parts: Vec<_> = input.split_whitespace().collect();
    let row_str = parts[parts.len() - 3];
    let row_str = &row_str[..row_str.len() - 1];
    let row = row_str.parse::<usize>().unwrap();
    let col_str = parts[parts.len() - 1];
    let col_str = &col_str[..col_str.len() - 1];
    let col = col_str.parse::<usize>().unwrap();
    (row, col)
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let (row, col) = parse_input(input);
        let num_iter = index_of(row - 1, col - 1);
        let mut v = 20151125u64;
        for _ in 0..num_iter {
            v = (v * 252533) % 33554393;
        }
        format!("{}", v)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(index_of(0, 0), 0);
        assert_eq!(index_of(1, 0), 1);
        assert_eq!(index_of(0, 1), 2);
        assert_eq!(index_of(2, 0), 3);
        assert_eq!(index_of(1, 1), 4);
        assert_eq!(index_of(0, 2), 5);
        assert_eq!(index_of(3, 0), 6);
        assert_eq!(index_of(2, 1), 7);
        assert_eq!(index_of(1, 2), 8);
        assert_eq!(index_of(0, 3), 9);
    }

    #[test]
    fn test_iter() {
        let mut v = 20151125u64;
        v = (v * 252533) % 33554393;
        assert_eq!(v, 31916031);
        v = (v * 252533) % 33554393;
        assert_eq!(v, 18749137);
    }
}
