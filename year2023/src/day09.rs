use common::day::Day;

pub struct Day09 {}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        parse_input(input)
            .map(|seq| extrapolate(&seq))
            .sum::<isize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        parse_input(input)
            .map(|mut seq| {
                seq.reverse();
                extrapolate(&seq)
            })
            .sum::<isize>()
            .to_string()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<isize>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}

fn extrapolate(seq: &[isize]) -> isize {
    let diff_seqs = diff_sequences(seq);

    let mut added_value = 0;
    for seq in diff_seqs.iter().rev() {
        added_value += seq.last().unwrap();
    }

    added_value
}

fn diff_sequences(seq: &[isize]) -> Vec<Vec<isize>> {
    let mut diff_seqs = vec![seq.to_vec()];

    loop {
        let last = diff_seqs.last().unwrap();
        let next: Vec<_> = last.windows(2).map(|el| el[1] - el[0]).collect();
        if next.iter().all(|v| *v == 0) {
            break;
        }
        diff_seqs.push(next);
    }

    diff_seqs
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn star1() {
        let d = Day09 {};
        assert_eq!(d.star1(INPUT), "114");
    }

    #[test]
    fn star2() {
        let d = Day09 {};
        assert_eq!(d.star2(INPUT), "2");
    }

    #[test]
    fn test_diff_sequences() {
        let diff_seqs = diff_sequences(&vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(diff_seqs.last().unwrap(), &vec![3, 3, 3, 3, 3]);

        let diff_seqs = diff_sequences(&vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(diff_seqs.last().unwrap(), &vec![1, 1, 1, 1]);

        let diff_seqs = diff_sequences(&vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(diff_seqs.last().unwrap(), &vec![2, 2, 2]);
    }

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
