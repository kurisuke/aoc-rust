use common::day::Day;

pub struct Day10 {}

fn step(input: &str) -> String {
    let mut output = vec![];
    let mut it = input.chars();
    let mut last_char = it.next().unwrap();
    let mut count = 1;
    for c in it {
        if c == last_char {
            count += 1;
        } else {
            output.push((count as u8 + 0x30) as char);
            output.push(last_char);
            last_char = c;
            count = 1;
        }
    }
    output.push((count as u8 + 0x30) as char);
    output.push(last_char);
    output.into_iter().collect()
}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let mut output = String::from(input);
        for _ in 0..40 {
            output = step(&output);
        }
        format!("{}", output.len())
    }

    fn star2(&self, input: &str) -> String {
        let mut output = String::from(input);
        for _ in 0..50 {
            output = step(&output);
        }
        format!("{}", output.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut output = String::from("1");
        for _ in 0..5 {
            output = step(&output);
        }
        assert_eq!(output, "312211");
    }
}
