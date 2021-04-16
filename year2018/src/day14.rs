use common::day::Day;

pub struct Day14 {}

struct State {
    pub scores: Vec<u8>,
    idx1: usize,
    idx2: usize,
}

impl State {
    fn iter(&mut self) {
        let new_score = self.scores[self.idx1] + self.scores[self.idx2];
        if new_score > 9 {
            self.scores.push(new_score / 10);
            self.scores.push(new_score % 10);
        } else {
            self.scores.push(new_score);
        }
        self.idx1 = (self.idx1 + self.scores[self.idx1] as usize + 1) % self.scores.len();
        self.idx2 = (self.idx2 + self.scores[self.idx2] as usize + 1) % self.scores.len();
    }
}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let max_recipes = input.trim().parse::<usize>().unwrap();
        let mut state = State {
            scores: vec![3, 7],
            idx1: 0,
            idx2: 1,
        };
        while state.scores.len() < max_recipes + 10 {
            state.iter();
        }
        let mut s = String::new();
        for i in 0..10 {
            s = format!("{}{}", s, state.scores[max_recipes + i]);
        }
        s
    }

    fn star2(&self, input: &str) -> String {
        let find_sequence: Vec<_> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let mut state = State {
            scores: vec![3, 7],
            idx1: 0,
            idx2: 1,
        };
        loop {
            state.iter();
            if state.scores.len() >= find_sequence.len()
                && state.scores[state.scores.len() - find_sequence.len()..state.scores.len()]
                    == find_sequence
            {
                return format!("{}", state.scores.len() - find_sequence.len());
            }
            if state.scores.len() > find_sequence.len()
                && state.scores
                    [state.scores.len() - find_sequence.len() - 1..state.scores.len() - 1]
                    == find_sequence
            {
                return format!("{}", state.scores.len() - find_sequence.len() - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day14 {};
        assert_eq!(d.star1("9"), "5158916779");
        assert_eq!(d.star1("5"), "0124515891");
        assert_eq!(d.star1("18"), "9251071085");
        assert_eq!(d.star1("2018"), "5941429882");
    }

    #[test]
    fn star2() {
        let d = Day14 {};
        assert_eq!(d.star2("51589"), "9");
        assert_eq!(d.star2("01245"), "5");
        assert_eq!(d.star2("92510"), "18");
        assert_eq!(d.star2("59414"), "2018");
    }
}
