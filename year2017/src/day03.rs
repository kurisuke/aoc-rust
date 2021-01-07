use common::day::Day;
use std::collections::HashMap;

pub struct Day03 {}

fn coords_nth(n: usize) -> (isize, isize) {
    if n == 1 {
        return (0, 0);
    }
    let (i, val) = last_corner(n);
    let corner_coords = coords_nth_corner(i);
    let off = (n - val) as isize;
    let dir = match i % 4 {
        1 => (0, off),
        2 => (-off, 0),
        3 => (0, -off),
        0 => (off, 0),
        _ => (0, 0),
    };
    (corner_coords.0 + dir.0, corner_coords.1 + dir.1)
}

fn coords_nth_corner(i: usize) -> (isize, isize) {
    let n = (i as isize - 1) / 4 + 1;
    match i % 4 {
        1 => (n, -(n - 1)),
        2 => (n, n),
        3 => (-n, n),
        0 => (-n, -n),
        _ => (0, 0),
    }
}

fn last_corner(n: usize) -> (usize, usize) {
    let mut sum = 1;
    let mut i = 0;
    let mut k = 1;
    while sum + k <= n {
        sum += k;
        i += 1;
        if i % 2 == 0 {
            k += 1;
        }
    }
    (i, sum)
}

fn dist(c: (isize, isize)) -> usize {
    c.0.abs() as usize + c.1.abs() as usize
}

fn neighbors(c: &(isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (c.0 + 1, c.1),
        (c.0 + 1, c.1 + 1),
        (c.0, c.1 + 1),
        (c.0 - 1, c.1 + 1),
        (c.0 - 1, c.1),
        (c.0 - 1, c.1 - 1),
        (c.0, c.1 - 1),
        (c.0 + 1, c.1 - 1),
    ]
}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let n = input.trim().parse::<usize>().unwrap();
        format!("{}", dist(coords_nth(n)))
    }

    fn star2(&self, input: &str) -> String {
        let max_written = input.trim().parse::<usize>().unwrap();
        let mut n = 1;
        let mut vals = HashMap::new();
        vals.insert((0, 0), 1);
        let mut last_written = 1;

        while last_written <= max_written {
            n += 1;
            let pos = coords_nth(n);
            last_written = neighbors(&pos)
                .iter()
                .map(|c| vals.get(c))
                .filter_map(|x| x)
                .sum::<usize>();
            vals.insert(pos, last_written);
        }
        format!("{}", last_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners() {
        assert_eq!(last_corner(2), (1, 2));
        assert_eq!(last_corner(3), (2, 3));
        assert_eq!(last_corner(5), (3, 5));
        assert_eq!(last_corner(6), (3, 5));
        assert_eq!(last_corner(19), (7, 17));
        assert_eq!(last_corner(26), (9, 26));

        assert_eq!(coords_nth_corner(1), (1, 0));
        assert_eq!(coords_nth_corner(4), (-1, -1));
        assert_eq!(coords_nth_corner(7), (-2, 2));
        assert_eq!(coords_nth_corner(10), (3, 3));
        assert_eq!(coords_nth_corner(13), (4, -3));

        assert_eq!(coords_nth(4), (0, 1));
        assert_eq!(coords_nth(8), (0, -1));
        assert_eq!(coords_nth(12), (2, 1));
        assert_eq!(coords_nth(16), (-1, 2));
        assert_eq!(coords_nth(20), (-2, -1));
        assert_eq!(coords_nth(24), (1, -2));
    }

    #[test]
    fn star1() {
        let d = Day03 {};
        assert_eq!(d.star1("1"), "0");
        assert_eq!(d.star1("2"), "1");
        assert_eq!(d.star1("12"), "3");
        assert_eq!(d.star1("23"), "2");
        assert_eq!(d.star1("1024"), "31");
    }
}
