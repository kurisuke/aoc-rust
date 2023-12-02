use common::day::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        const RED: usize = 12;
        const GREEN: usize = 13;
        const BLUE: usize = 14;

        let games = parse_input(input);
        games
            .iter()
            .filter(|game| game.possible(RED, GREEN, BLUE))
            .map(|game| game.id)
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let games = parse_input(input);
        games
            .iter()
            .map(|game| {
                let min_cubes = game.min_cubes();
                min_cubes.red * min_cubes.green * min_cubes.blue
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<CubeSubset>,
}

impl Game {
    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.subsets
            .iter()
            .all(|subset| red >= subset.red && green >= subset.green && blue >= subset.blue)
    }

    fn min_cubes(&self) -> CubeSubset {
        let mut min_cubes = CubeSubset {
            red: 0,
            green: 0,
            blue: 0,
        };

        for subset in self.subsets.iter() {
            min_cubes.red = min_cubes.red.max(subset.red);
            min_cubes.green = min_cubes.green.max(subset.green);
            min_cubes.blue = min_cubes.blue.max(subset.blue);
        }

        min_cubes
    }
}

#[derive(Debug)]
struct CubeSubset {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut spl_colon = line.split(": ");

            let id_part = spl_colon.next().unwrap();
            let subsets_part = spl_colon.next().unwrap();
            let id: usize = id_part.split(' ').nth(1).unwrap().parse().unwrap();

            let subsets = subsets_part
                .split("; ")
                .map(|subsets_str| {
                    let mut cube_subset = CubeSubset {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };

                    for subset_str in subsets_str.split(", ") {
                        let mut spl_subset = subset_str.split(' ');
                        let count: usize = spl_subset.next().unwrap().parse().unwrap();
                        let color = spl_subset.next().unwrap();
                        match color {
                            "red" => {
                                cube_subset.red = count;
                            }
                            "green" => {
                                cube_subset.green = count;
                            }
                            "blue" => {
                                cube_subset.blue = count;
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }

                    cube_subset
                })
                .collect();

            Game { id, subsets }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn star1() {
        let d = Day02 {};
        assert_eq!(d.star1(INPUT), "8");
    }

    #[test]
    fn star2() {
        let d = Day02 {};
        assert_eq!(d.star2(INPUT), "2286");
    }
}
