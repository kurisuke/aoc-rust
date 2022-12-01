use common::day::Day;
use regex::Regex;
use std::cmp::Ordering;
use util::chardistrib::char_distribution;

pub struct Day04 {}

struct Room<'a> {
    encr_name: &'a str,
    sector_id: usize,
    checksum: &'a str,
}

fn calc_checksum(s: &str) -> String {
    let freq = char_distribution(&s.replace('-', ""));
    let mut freq: Vec<(_, _)> = freq.iter().collect();
    freq.sort_by(|a, b| match a.1.cmp(b.1) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => a.0.cmp(b.0),
    });
    freq[0..5].iter().map(|x| x.0).collect()
}

fn parse_input(input: &str) -> Vec<Room> {
    let re = Regex::new(r"([a-z\-]+)-([0-9]+)\[([a-z]{5})\]").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let encr_name = caps.get(1).unwrap().as_str();
            let sector_id = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let checksum = caps.get(3).unwrap().as_str();
            Room {
                encr_name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

fn rotate_char(c: char, offset: usize) -> char {
    let alphabet_pos = c as u8 - b'a';
    let rotated_pos = (alphabet_pos as usize + offset) % 26;
    (rotated_pos as u8 + b'a') as char
}

fn decrypt_name(room: &Room) -> String {
    room.encr_name
        .chars()
        .map(|c| match c {
            'a'..='z' => rotate_char(c, room.sector_id),
            '-' => ' ',
            _ => {
                panic!("unexpected character: {}", c);
            }
        })
        .collect()
}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let rooms = parse_input(input);
        let sum_sector_ids = rooms
            .iter()
            .filter(|room| room.checksum == calc_checksum(room.encr_name))
            .map(|room| room.sector_id)
            .sum::<usize>();
        format!("{}", sum_sector_ids)
    }

    fn star2(&self, input: &str) -> String {
        let rooms = parse_input(input);
        let valid_rooms: Vec<_> = rooms
            .into_iter()
            .filter(|room| room.checksum == calc_checksum(room.encr_name))
            .collect();
        let mut northpole_rooms = valid_rooms
            .iter()
            .filter(|room| decrypt_name(room).contains("northpole"));
        format!("{}", northpole_rooms.next().unwrap().sector_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day04 {};
        let input = r#"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"#;
        assert_eq!(d.star1(input), "1514");
    }

    #[test]
    fn ex2() {
        let input = r#"qzmt-zixmtkozy-ivhz-343[abcde]"#;
        let rooms = parse_input(input);
        assert_eq!(
            decrypt_name(rooms.iter().next().unwrap()),
            "very encrypted name"
        );
    }
}
