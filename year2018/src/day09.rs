use common::day::Day;
use std::collections::VecDeque;

pub struct Day09 {}

struct Game {
    pub circle: VecDeque<usize>,
    pub player_scores: Vec<usize>,
}

impl Game {
    fn new(num_players: usize) -> Game {
        let mut circle = VecDeque::new();
        circle.push_back(0);
        Game {
            circle,
            player_scores: vec![0; num_players],
        }
    }

    fn place(&mut self, num: usize) {
        self.circle.rotate_left(2 % self.circle.len());
        self.circle.push_front(num);
    }

    fn score(&mut self, num: usize) {
        let scoring_player = num % self.player_scores.len();
        self.player_scores[scoring_player] += num;
        self.circle.rotate_right(7 % self.circle.len());
        let add_score = self.circle.pop_front().unwrap();
        self.player_scores[scoring_player] += add_score;
    }

    fn run(&mut self, last_marble: usize) -> usize {
        for n in 1..=last_marble {
            if n % 23 > 0 {
                self.place(n);
            } else {
                self.score(n);
            }
        }
        *self.player_scores.iter().max().unwrap()
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let l = input.lines().next().unwrap();
    let w: Vec<_> = l.split_whitespace().collect();
    (
        w[0].parse::<usize>().unwrap(),
        w[6].parse::<usize>().unwrap(),
    )
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let (num_players, last_marble) = parse_input(input);
        let mut g = Game::new(num_players);
        format!("{}", g.run(last_marble))
    }

    fn star2(&self, input: &str) -> String {
        let (num_players, last_marble) = parse_input(input);
        let mut g = Game::new(num_players);
        format!("{}", g.run(last_marble * 100))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day09 {};
        assert_eq!(
            d.star1("10 players; last marble is worth 1618 points"),
            "8317"
        );
        assert_eq!(
            d.star1("13 players; last marble is worth 7999 points"),
            "146373"
        );
        assert_eq!(
            d.star1("17 players; last marble is worth 1104 points"),
            "2764"
        );
        assert_eq!(
            d.star1("21 players; last marble is worth 6111 points"),
            "54718"
        );
        assert_eq!(
            d.star1("30 players; last marble is worth 5807 points"),
            "37305"
        );
    }
}
