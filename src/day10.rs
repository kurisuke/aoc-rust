use crate::day::Day;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

pub struct Day10 {}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let mut adapter_ratings: Vec<u64> = vec![0];
        let adapter_ratings_input: Vec<_> = input
            .lines()
            .map(|l| l.parse::<u64>())
            .filter_map(Result::ok)
            .sorted()
            .collect();
        adapter_ratings.extend(adapter_ratings_input);
        adapter_ratings.push(adapter_ratings.last().unwrap() + 3);

        let mut count_1 = 0;
        let mut count_3 = 0;
        for j in adapter_ratings.windows(2) {
            match j[1] - j[0] {
                1 => {
                    count_1 += 1;
                }
                3 => {
                    count_3 += 1;
                }
                _ => {}
            }
        }
        format!("{}", count_1 * count_3)
    }

    fn star2(&self, input: &str) -> String {
        let adapter_ratings: BTreeSet<_> = input
            .lines()
            .map(|l| l.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        let mut num_paths = HashMap::new();

        // starting jolt level 0
        num_paths.insert(0i64, 1i64);

        for j in adapter_ratings.iter() {
            // max 3 jolts difference between adapters means:
            // paths to jolt level = sum of paths to j-1, j-2, j-3
            num_paths.insert(
                *j,
                (1..=3).map(|x| num_paths.get(&(j - x)).unwrap_or(&0)).sum(),
            );
        }
        format!("{}", num_paths[adapter_ratings.iter().max().unwrap()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
16
10
15
5
1
11
7
19
6
12
4"#;
        let d = Day10 {};
        assert_eq!(d.star1(input), format!("{}", 7 * 5));
        assert_eq!(d.star2(input), "8");
    }

    #[test]
    fn ex2() {
        let input = r#"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;
        let d = Day10 {};
        assert_eq!(d.star1(input), format!("{}", 22 * 10));
        assert_eq!(d.star2(input), "19208");
    }
}
