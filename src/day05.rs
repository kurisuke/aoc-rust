use crate::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let seat_ids: Vec<u16> = input.lines().map(|l| seat_str_to_id(l)).collect();
        format!("{}", seat_ids.iter().max().unwrap())
    }

    fn star2(&self, input: &str) -> String {
        let mut seat_ids: Vec<u16> = input.lines().map(|l| seat_str_to_id(l)).collect();
        seat_ids.sort();
        let my_seat_id = seat_ids
            .windows(2)
            .filter(|&s| s[1] - s[0] > 1)
            .map(|s| s[0] + 1)
            .next()
            .unwrap();
        format!("{}", my_seat_id)
    }
}

fn seat_str_to_id(seat_str: &str) -> u16 {
    let fb_str = String::from(&seat_str[0..7]);
    let lr_str = String::from(&seat_str[7..10]);

    u16::from_str_radix(&fb_str.replace("F", "0").replace("B", "1"), 2).unwrap() * 8
        + u16::from_str_radix(&lr_str.replace("L", "0").replace("R", "1"), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day05 {};

        let input = "FBFBBFFRLR";
        assert_eq!(d.star1(input), "357");

        let input = "BFFFBBFRRR";
        assert_eq!(d.star1(input), "567");

        let input = "FFFBBBFRRR";
        assert_eq!(d.star1(input), "119");

        let input = "BBFFBBFRLL";
        assert_eq!(d.star1(input), "820");
    }
}
