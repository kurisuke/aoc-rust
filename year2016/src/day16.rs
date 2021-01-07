use common::day::Day;

pub struct Day16 {}

fn fill_disk(input: &[bool], req_size: usize) -> Vec<bool> {
    let mut output: Vec<_> = input.iter().copied().collect();
    while output.len() < req_size {
        let rev: Vec<_> = output.iter().rev().copied().collect();
        let inv: Vec<_> = rev.into_iter().map(|x| !x).collect();
        output.push(false);
        output.extend(inv);
    }
    output.truncate(req_size);
    output
}

fn checksum(data: &[bool]) -> String {
    let mut checksum: Vec<_> = data.iter().copied().collect();
    while checksum.len() % 2 == 0 {
        checksum = checksum.chunks(2).map(|c| c[0] == c[1]).collect();
    }
    checksum
        .iter()
        .map(|v| if *v { '1' } else { '0' })
        .collect()
}

fn parse_input(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect()
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let input = parse_input(input);
        let fill_data = fill_disk(&input, 272);
        checksum(&fill_data)
    }

    fn star2(&self, input: &str) -> String {
        let input = parse_input(input);
        let fill_data = fill_disk(&input, 35651584);
        checksum(&fill_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = parse_input("10000");
        let fill_data = fill_disk(&input, 20);
        assert_eq!(checksum(&fill_data), "01100");
    }
}
