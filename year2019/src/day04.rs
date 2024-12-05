use common::day::Day;

pub struct Day04 {}

fn digits(n: &u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect()
}

fn check_adj2(ds: &[u32]) -> bool {
    let mut len = 1;
    for d in ds.windows(2) {
        if d[0] == d[1] {
            len += 1;
        } else {
            if len == 2 {
                return true;
            }
            len = 1;
        }
    }
    len == 2
}

fn is_valid_pt1(n: &u32) -> bool {
    let ds = digits(n);
    let check_adj = ds.windows(2).any(|d| d[0] == d[1]);
    let check_asc = ds.windows(2).all(|d| d[1] >= d[0]);
    check_adj && check_asc
}

fn is_valid_pt2(n: &u32) -> bool {
    let ds = digits(n);
    let check_asc = ds.windows(2).all(|d| d[1] >= d[0]);
    check_adj2(&ds) && check_asc
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let range: Vec<_> = input
            .lines()
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let count = (range[0]..=range[1]).filter(is_valid_pt1).count();
        format!("{}", count)
    }

    fn star2(&self, input: &str) -> String {
        let range: Vec<_> = input
            .lines()
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let count = (range[0]..=range[1]).filter(is_valid_pt2).count();
        format!("{}", count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        assert!(is_valid_pt1(&111111));
        assert!(!is_valid_pt1(&223450));
        assert!(!is_valid_pt1(&123789));
    }

    #[test]
    fn star2() {
        assert!(is_valid_pt2(&112233));
        assert!(!is_valid_pt2(&123444));
        assert!(is_valid_pt2(&111122));
    }
}
