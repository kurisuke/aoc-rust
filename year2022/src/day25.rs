use common::day::Day;

pub struct Day25 {}

fn to_snafu(n: isize) -> String {
    let mut acc = n;
    let mut v = vec![];
    while acc > 0 {
        let tmp = (acc + 2) / 5;
        let rem = (acc + 2) % 5;
        acc = tmp;
        v.push(match rem {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!(),
        })
    }
    v.into_iter().rev().collect()
}

fn from_snafu(s: &str) -> isize {
    let mut p = 1;
    let mut a = 0;
    for c in s.chars().rev() {
        a += p * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };
        p *= 5;
    }
    a
}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let sum = input.lines().map(from_snafu).sum();
        to_snafu(sum)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");

        assert_eq!(from_snafu("1=-0-2"), 1747);
        assert_eq!(from_snafu("12111"), 906);
        assert_eq!(from_snafu("2=0="), 198);

        let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

        let d = Day25 {};
        assert_eq!(d.star1(input), "2=-1=0");
    }
}
