use common::day::Day;

pub struct Day04 {}

#[derive(Clone, Copy)]
struct BoardField {
    pub number: u64,
    pub marked: bool,
}

type BingoBoard = [[BoardField; 5]; 5];

const BINGO_SIZE: usize = 5;

fn parse_input(input: &str) -> (Vec<u64>, Vec<BingoBoard>) {
    let numbers_str = input.split("\n\n").next().unwrap();
    let numbers = numbers_str.split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards = vec![];
    for board_str in input.split("\n\n").skip(1) {
        let mut board = [[BoardField {
            number: 0,
            marked: false,
        }; BINGO_SIZE]; BINGO_SIZE];
        for (y, l) in board_str.lines().enumerate() {
            for (x, n) in l.split_whitespace().enumerate() {
                board[x][y].number = n.parse().unwrap();
            }
        }
        boards.push(board);
    }
    (numbers, boards)
}

fn play_bingo(numbers: &[u64], mut boards: Vec<BingoBoard>) -> Option<(u64, BingoBoard)> {
    for new_number in numbers {
        for board in boards.iter_mut() {
            if check_board(board, *new_number) {
                return Some((*new_number, *board));
            }
        }
    }
    None
}

fn check_board(board: &mut BingoBoard, new_number: u64) -> bool {
    let mut result = false;
    for x in 0..BINGO_SIZE {
        result |= check_row(board, new_number, x);
    }
    if !result {
        for y in 0..BINGO_SIZE {
            result |= check_col(board, new_number, y);
        }
    }
    result
}

fn check_row(board: &mut BingoBoard, new_number: u64, x: usize) -> bool {
    for y in 0..BINGO_SIZE {
        if !(board[x][y].marked) {
            if board[x][y].number == new_number {
                board[x][y].marked = true;
            } else {
                return false;
            }
        }
    }
    true
}

fn check_col(board: &mut BingoBoard, new_number: u64, y: usize) -> bool {
    let mut result = true;
    for x in 0..BINGO_SIZE {
        if !(board[x][y].marked) {
            if board[x][y].number == new_number {
                board[x][y].marked = true;
            } else {
                result = false;
            }
        }
    }
    result
}

fn calc_score(win_number: u64, board: &BingoBoard) -> u64 {
    let mut sum = 0;
    for x in 0..BINGO_SIZE {
        for y in 0..BINGO_SIZE {
            if !(board[x][y].marked) {
                sum += board[x][y].number;
            }
        }
    }
    sum * win_number
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let (numbers, boards) = parse_input(input);
        let (win_number, board) = play_bingo(&numbers, boards).unwrap();
        let score = calc_score(win_number, &board);
        format!("{}", score)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let d = Day04 {};
        assert_eq!(d.star1(input), "4512");
    }
}
