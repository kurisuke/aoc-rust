use common::day::Day;

pub struct Day03 {}

fn possible_triangle(x: &[usize]) -> bool {
    x[0] + x[1] > x[2] && x[1] + x[2] > x[0] && x[2] + x[0] > x[1]
}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .lines()
                .map(|line| line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>())
                .filter(|x| possible_triangle(x))
                .count()
        )
    }

    fn star2(&self, input: &str) -> String {
        let nums: Vec<_> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut triangles = 0;
        for i in 0..(nums.len() / 3) {
            #[allow(clippy::needless_range_loop)]
            for j in 0..=2 {
                if possible_triangle(&[nums[3 * i][j], nums[3 * i + 1][j], nums[3 * i + 2][j]]) {
                    triangles += 1;
                }
            }
        }
        format!("{}", triangles)
    }
}
