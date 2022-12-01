use common::day::Day;

pub struct Day15 {}

struct IngredStats {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_input(input: &str) -> Vec<IngredStats> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let capacity = parts[2].replace(',', "").parse::<i64>().unwrap();
            let durability = parts[4].replace(',', "").parse::<i64>().unwrap();
            let flavor = parts[6].replace(',', "").parse::<i64>().unwrap();
            let texture = parts[8].replace(',', "").parse::<i64>().unwrap();
            let calories = parts[10].replace(',', "").parse::<i64>().unwrap();
            IngredStats {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect()
}

fn get_ratios(sum: usize, n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        vec![vec![sum]]
    } else {
        let mut ret = vec![];
        for i in 0..=sum {
            let entry = vec![i];
            for j in get_ratios(sum - i, n - 1) {
                let mut e = entry.clone();
                e.extend(j);
                ret.push(e);
            }
        }
        ret
    }
}

fn best_score(stats: &[IngredStats], ratios: &[Vec<usize>]) -> i64 {
    let mut best_score = 0;
    for ratio in ratios {
        let capacity = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.capacity * *r as i64)
            .sum::<i64>()
            .max(0);
        let durability = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.durability * *r as i64)
            .sum::<i64>()
            .max(0);
        let flavor = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.flavor * *r as i64)
            .sum::<i64>()
            .max(0);
        let texture = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.texture * *r as i64)
            .sum::<i64>()
            .max(0);
        let score = capacity * durability * flavor * texture;
        best_score = best_score.max(score);
    }
    best_score
}

fn best_score_with_calories(
    stats: &[IngredStats],
    ratios: &[Vec<usize>],
    calories_target: i64,
) -> i64 {
    let mut best_score = 0;
    for ratio in ratios {
        let capacity = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.capacity * *r as i64)
            .sum::<i64>()
            .max(0);
        let durability = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.durability * *r as i64)
            .sum::<i64>()
            .max(0);
        let flavor = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.flavor * *r as i64)
            .sum::<i64>()
            .max(0);
        let texture = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.texture * *r as i64)
            .sum::<i64>()
            .max(0);
        let calories = stats
            .iter()
            .zip(ratio.iter())
            .map(|(s, r)| s.calories * *r as i64)
            .sum::<i64>()
            .max(0);
        if calories_target == calories {
            let score = capacity * durability * flavor * texture;
            best_score = best_score.max(score);
        }
    }
    best_score
}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let stats = parse_input(input);
        let ratios = get_ratios(100, stats.len());
        format!("{}", best_score(&stats, &ratios))
    }

    fn star2(&self, input: &str) -> String {
        let stats = parse_input(input);
        let ratios = get_ratios(100, stats.len());
        format!("{}", best_score_with_calories(&stats, &ratios, 500))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;
        let d = Day15 {};
        assert_eq!(d.star1(input), "62842880");
        assert_eq!(d.star2(input), "57600000");
    }
}
