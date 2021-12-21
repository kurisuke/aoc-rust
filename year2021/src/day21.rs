use common::day::Day;
use std::collections::HashMap;

pub struct Day21 {}

struct DetDie {
    last_roll: usize,
    sides: usize,
    roll_count: usize,
}

impl DetDie {
    fn new(sides: usize) -> DetDie {
        DetDie {
            last_roll: 0,
            sides,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.last_roll = (self.last_roll + 1) % self.sides;
        self.roll_count += 1;
        if self.last_roll == 0 {
            self.sides
        } else {
            self.last_roll
        }
    }

    fn roll_n(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.roll()).sum()
    }
}

const DIRAC_3ROLLS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

#[derive(PartialEq, Eq, Hash, Clone)]
struct Universe {
    pos: [usize; 2],
    score: [usize; 2],
}

#[derive(Eq, PartialEq)]
enum UniverseState {
    P1Win,
    P2Win,
    Undecided,
}

impl Universe {
    fn state(&self) -> UniverseState {
        if self.score[0] >= 21 {
            UniverseState::P1Win
        } else if self.score[1] >= 21 {
            UniverseState::P2Win
        } else {
            UniverseState::Undecided
        }
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse().unwrap())
        .collect()
}

fn play(start: &[usize]) -> usize {
    let mut cur = 0;
    let mut score = [0, 0];
    let mut pos = start.to_vec();
    let mut die = DetDie::new(100);
    while score[0] < 1000 && score[1] < 1000 {
        pos[cur] = (pos[cur] + die.roll_n(3)) % 10;
        score[cur] += if pos[cur] == 0 { 10 } else { pos[cur] };

        cur = (cur + 1) % 2;
    }
    die.roll_count * score[0].min(score[1])
}

fn dirac_turn(universes: &HashMap<Universe, usize>, cur: usize) -> HashMap<Universe, usize> {
    let mut next_universes = HashMap::new();
    for (universe, n) in universes {
        for (i, k) in DIRAC_3ROLLS.iter().enumerate() {
            let roll = i + 3;
            let mut new_universe = universe.clone();
            new_universe.pos[cur] = (new_universe.pos[cur] + roll) % 10;
            new_universe.score[cur] += if new_universe.pos[cur] == 0 {
                10
            } else {
                new_universe.pos[cur]
            };
            *next_universes.entry(new_universe).or_insert(0) += n * k;
        }
    }
    next_universes
}

fn play_pt2(start: &[usize]) -> usize {
    let mut universes = HashMap::new();
    universes.insert(
        Universe {
            pos: [start[0], start[1]],
            score: [0, 0],
        },
        1,
    );

    let mut cur = 0;
    let mut wins = [0, 0];
    loop {
        let universes_tmp = dirac_turn(&universes, cur);
        let win_state = if cur == 0 {
            UniverseState::P1Win
        } else {
            UniverseState::P2Win
        };
        let universes_win = universes_tmp
            .iter()
            .filter(|(k, _)| k.state() == win_state)
            .map(|(_, v)| v)
            .sum::<usize>();
        wins[cur] += universes_win;
        universes = universes_tmp
            .into_iter()
            .filter(|(k, _)| k.state() == UniverseState::Undecided)
            .collect();
        if universes.is_empty() {
            break;
        } else {
            cur = (cur + 1) % 2;
        }
    }
    wins[0].max(wins[1])
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let start = parse_input(input);
        format!("{}", play(&start))
    }

    fn star2(&self, input: &str) -> String {
        let start = parse_input(input);
        format!("{}", play_pt2(&start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

        let d = Day21 {};
        assert_eq!(d.star1(input), "739785");
        assert_eq!(d.star2(input), "444356092776315");
    }
}
