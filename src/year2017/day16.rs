use crate::day::Day;

pub struct Day16 {}

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .trim()
        .split(',')
        .map(|tok| match tok.chars().next().unwrap() {
            's' => Move::Spin(tok[1..].parse::<usize>().unwrap()),
            'x' => {
                let param_tok: Vec<_> = tok[1..].split('/').collect();
                Move::Exchange(
                    param_tok[0].parse::<usize>().unwrap(),
                    param_tok[1].parse::<usize>().unwrap(),
                )
            }
            'p' => {
                let cs: Vec<_> = tok.chars().collect();
                Move::Partner(cs[1], cs[3])
            }
            _ => {
                panic!("Cannot parse token: {}", tok);
            }
        })
        .collect()
}

fn apply(progs: &mut Vec<char>, mov: &Move) {
    let plen = progs.len();
    match mov {
        Move::Spin(x) => {
            progs.rotate_right(x % plen);
        }
        Move::Exchange(x, y) => {
            progs.swap(*x, *y);
        }
        Move::Partner(a, b) => {
            let a = progs.iter().position(|x| x == a).unwrap();
            let b = progs.iter().position(|x| x == b).unwrap();
            progs.swap(a, b);
        }
    }
}

fn dance(prog_str: &str, moves: &[Move]) -> String {
    let mut progs: Vec<_> = prog_str.chars().collect();
    for mov in moves.iter() {
        apply(&mut progs, mov);
    }
    progs.into_iter().collect()
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let moves = parse_input(input);
        dance("abcdefghijklmnop", &moves)
    }

    fn star2(&self, input: &str) -> String {
        let moves = parse_input(input);

        let mut repeat_len = 0u64;
        let start = "abcdefghijklmnop";
        let mut progs = start.to_string();
        while start != progs || repeat_len == 0 {
            repeat_len += 1;
            progs = dance(&progs, &moves);
        }

        let offset = 1_000_000_000 % repeat_len;
        progs = start.to_string();
        for _ in 0..offset {
            progs = dance(&progs, &moves);
        }
        progs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "s1,x3/4,pe/b";
        let moves = parse_input(input);
        assert_eq!(dance("abcde", &moves), "baedc");
    }
}
