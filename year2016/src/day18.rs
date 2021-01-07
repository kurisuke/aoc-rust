use common::day::Day;

pub struct Day18 {}

// Represent the tiles as booleans:
// . -> safe == true
// ^ -> trap == false

fn parse_input(input: &str) -> Vec<bool> {
    input.trim().chars().map(|c| c == '.').collect()
}

fn next_row(row: &[bool]) -> Vec<bool> {
    (0..row.len())
        .map(|i| {
            let check_tiles = if i == 0 {
                (true, row[i + 1])
            } else if i == row.len() - 1 {
                (row[i - 1], true)
            } else {
                (row[i - 1], row[i + 1])
            };
            !(check_tiles.0 ^ check_tiles.1)
        })
        .collect()
}

fn safe_tiles(start_row: Vec<bool>, num_rows: usize) -> usize {
    let mut row = start_row;
    let mut safe_tiles = row.iter().filter(|&&c| c).count();
    for _ in 1..num_rows {
        row = next_row(&row);
        safe_tiles += row.iter().filter(|&&c| c).count();
    }
    safe_tiles
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let start_row = parse_input(input);
        format!("{}", safe_tiles(start_row, 40))
    }

    fn star2(&self, input: &str) -> String {
        let start_row = parse_input(input);
        format!("{}", safe_tiles(start_row, 400000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let start_row = parse_input(".^^.^.^^^^");
        assert_eq!(safe_tiles(start_row, 10), 38);
    }
}
