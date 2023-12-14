use std::{collections::HashMap, iter};

use common::day::Day;

pub struct Day12 {}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Record {
    springs: String,
    sizes: Vec<usize>,
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let mut cache = HashMap::new();

        input
            .lines()
            .map(Record::parse)
            .map(|record| record.combinations(&mut cache))
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut cache = HashMap::new();

        input
            .lines()
            .map(Record::parse)
            .map(|record| record.unfold(5))
            .map(|record| record.combinations(&mut cache))
            .sum::<usize>()
            .to_string()
    }
}

impl Record {
    fn parse(line: &str) -> Record {
        let (springs, sizes) = line.split_once(' ').unwrap();
        let sizes = sizes.split(',').map(|n| n.parse().unwrap()).rev().collect();

        Record {
            springs: springs.to_string(),
            sizes,
        }
    }

    fn unfold(self, n: usize) -> Record {
        let springs: Vec<_> = iter::repeat(self.springs).take(n).collect();
        let sizes = iter::repeat(self.sizes).take(n).flatten().collect();
        Record {
            springs: springs.join("?"),
            sizes,
        }
    }

    fn combinations(mut self, cache: &mut HashMap<Record, usize>) -> usize {
        if cache.contains_key(&self) {
            return *cache.get(&self).unwrap();
        }

        // println!("combinations: {} {:?}", self.springs, self.sizes);
        if self.springs.is_empty() {
            if self.sizes.is_empty() {
                cache.insert(self, 1);
                1
            } else {
                cache.insert(self, 0);
                0
            }
        } else {
            match self.springs.chars().next().unwrap() {
                '.' => {
                    self.springs.remove(0);
                    let c = self.clone().combinations(cache);
                    cache.insert(self, c);
                    c
                }
                '#' => {
                    if !self.sizes.is_empty() {
                        let n = *self.sizes.last().unwrap();
                        if (0..n).all(|i| {
                            self.springs
                                .chars()
                                .nth(i)
                                .is_some_and(|c| c == '#' || c == '?')
                        }) && !self.springs.chars().nth(n).is_some_and(|c| c == '#')
                        {
                            self.springs = self.springs.chars().skip(n).collect();
                            self.sizes.pop();

                            match self.springs.chars().next() {
                                None => {}
                                Some('.') => {
                                    self.springs.remove(0);
                                }
                                Some('?') => {
                                    self.springs.replace_range(0..1, ".");
                                }
                                Some('#') => unreachable!(),
                                _ => unreachable!(),
                            }

                            let c = self.clone().combinations(cache);
                            cache.insert(self, c);
                            c
                        } else {
                            cache.insert(self, 0);
                            0
                        }
                    } else {
                        cache.insert(self, 0);
                        0
                    }
                }
                '?' => {
                    let mut springs1 = self.springs.clone();
                    springs1.replace_range(0..1, "#");
                    let rec1 = Record {
                        springs: springs1,
                        sizes: self.sizes.clone(),
                    };
                    self.springs.replace_range(0..1, ".");
                    let c1 = rec1.clone().combinations(cache);
                    let c2 = self.clone().combinations(cache);
                    cache.insert(rec1, c1);
                    cache.insert(self, c2);
                    c1 + c2
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn ex1() {
        let d = Day12 {};
        assert_eq!(d.star1(INPUT), "21");
    }

    #[test]
    fn ex2() {
        let d = Day12 {};
        assert_eq!(d.star2(INPUT), "525152");
    }

    #[test]
    fn unfold() {
        let record = Record::parse("???.### 1,1,3");
        let mut unfolded = record.unfold(5);
        assert_eq!(unfolded.springs, "???.###????.###????.###????.###????.###");
        unfolded.sizes.reverse();
        assert_eq!(
            unfolded.sizes,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }
}
