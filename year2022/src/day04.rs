use common::day::Day;

pub struct Day04 {}

struct Assignment(usize, usize);
struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    fn has_contained(&self) -> bool {
        (self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1)
            || (self.1 .0 <= self.0 .0 && self.1 .1 >= self.0 .1)
    }

    fn has_overlap(&self) -> bool {
        self.0 .0.max(self.1 .0) <= self.0 .1.min(self.1 .1)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = AssignmentPair> + '_ {
    input.lines().map(|l| {
        let mut iter = l.split(&[',', '-']);
        AssignmentPair(
            Assignment(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ),
            Assignment(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ),
        )
    })
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        format!(
            "{}",
            parse_input(input).filter(|a| a.has_contained()).count()
        )
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", parse_input(input).filter(|a| a.has_overlap()).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let d = Day04 {};
        assert_eq!(d.star1(input), "2");
        assert_eq!(d.star2(input), "4");
    }
}
