use common::day::Day;

pub struct Day02 {}

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from(c: char) -> Move {
        match c {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Unexpected character: {}", c),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_move(mov: &Move) -> Outcome {
        match mov {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

struct Strategy(Move, Move);

impl Strategy {
    fn score(&self) -> usize {
        #[allow(clippy::identity_op)]
        match (&self.0, &self.1) {
            // points for move + points for outcome
            (Move::Rock, Move::Rock) => 1 + 3,
            (Move::Rock, Move::Paper) => 1 + 0,
            (Move::Rock, Move::Scissors) => 1 + 6,
            (Move::Paper, Move::Rock) => 2 + 6,
            (Move::Paper, Move::Paper) => 2 + 3,
            (Move::Paper, Move::Scissors) => 2 + 0,
            (Move::Scissors, Move::Rock) => 3 + 0,
            (Move::Scissors, Move::Paper) => 3 + 6,
            (Move::Scissors, Move::Scissors) => 3 + 3,
        }
    }

    fn score_pt2(&self) -> usize {
        // second element is actually the desired outcome
        let outcome = Outcome::from_move(&self.1);
        // get required move for the desired outcome
        let own_move = match (&outcome, &self.0) {
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
            (Outcome::Draw, opp) => *opp,
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
        };
        let strategy = Strategy(own_move, self.1);
        strategy.score()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Strategy> + '_ {
    input.lines().map(|l| {
        let mut c = l.chars();
        let opp_move = Move::from(c.next().unwrap());
        let own_move = Move::from(c.nth(1).unwrap());
        Strategy(own_move, opp_move)
    })
}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        format!("{}", parse_input(input).map(|s| s.score()).sum::<usize>())
    }

    fn star2(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input).map(|s| s.score_pt2()).sum::<usize>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"A Y
B X
C Z"#;
        let d = Day02 {};
        assert_eq!(d.star1(input), "15");
        assert_eq!(d.star2(input), "12");
    }
}
