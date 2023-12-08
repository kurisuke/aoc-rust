use std::{cmp::Ordering, collections::HashMap};

use common::day::Day;

pub struct Day07 {}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let mut hands: Vec<_> = input.lines().map(HandWithBid::parse).collect();
        hands.sort_by_key(|h| h.hand);
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum::<usize>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut hands: Vec<_> = input
            .lines()
            .map(HandWithBid::parse)
            .map(HandWithBidPt2::from_part1)
            .collect();
        hands.sort_by_key(|h| h.hand);
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum::<usize>()
            .to_string()
    }
}

struct HandWithBid {
    hand: Hand,
    bid: usize,
}

impl HandWithBid {
    fn parse(line: &str) -> HandWithBid {
        let mut spl = line.split_whitespace();
        let hand = Hand::parse(spl.next().unwrap());
        let bid = spl.next().unwrap().parse().unwrap();
        HandWithBid { hand, bid }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        let cards = input
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Hand {
            cards,
            hand_type: HandType::from(&cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards[0].cmp(&other.cards[0]))
            .then_with(|| self.cards[1].cmp(&other.cards[1]))
            .then_with(|| self.cards[2].cmp(&other.cards[2]))
            .then_with(|| self.cards[3].cmp(&other.cards[3]))
            .then_with(|| self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandType {
    fn from(cards: &[Card; 5]) -> HandType {
        let mut freq = HashMap::new();
        for c in cards {
            let e = freq.entry(c).or_insert(0);
            *e += 1;
        }

        if freq.values().any(|v| *v == 5) {
            HandType::Five
        } else if freq.values().any(|v| *v == 4) {
            HandType::Four
        } else if freq.values().any(|v| *v == 3) && freq.values().any(|v| *v == 2) {
            HandType::FullHouse
        } else if freq.values().any(|v| *v == 3) {
            HandType::Three
        } else if freq.values().filter(|v| **v == 2).count() == 2 {
            HandType::TwoPair
        } else if freq.values().any(|v| *v == 2) {
            HandType::OnePair
        } else {
            HandType::High
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::N2),
            '3' => Ok(Card::N3),
            '4' => Ok(Card::N4),
            '5' => Ok(Card::N5),
            '6' => Ok(Card::N6),
            '7' => Ok(Card::N7),
            '8' => Ok(Card::N8),
            '9' => Ok(Card::N9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err("invalid char"),
        }
    }
}

// PART 2

struct HandWithBidPt2 {
    hand: HandPt2,
    bid: usize,
}

impl HandWithBidPt2 {
    fn from_part1(hand_with_bit_pt1: HandWithBid) -> HandWithBidPt2 {
        HandWithBidPt2 {
            hand: HandPt2::from_part1(hand_with_bit_pt1.hand),
            bid: hand_with_bit_pt1.bid,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct HandPt2 {
    cards: [CardPt2; 5],
    hand_type: HandType,
}

impl HandPt2 {
    fn from_part1(hand_pt1: Hand) -> HandPt2 {
        let other_cards = [
            Card::N2,
            Card::N3,
            Card::N4,
            Card::N5,
            Card::N6,
            Card::N7,
            Card::N8,
            Card::N9,
            Card::T,
            Card::Q,
            Card::K,
            Card::A,
        ];

        let hand_type = if hand_pt1.cards.iter().any(|c| *c == Card::J) {
            other_cards
                .into_iter()
                .map(|o| {
                    let cards_wildcard: [Card; 5] = hand_pt1
                        .cards
                        .iter()
                        .map(|c| if *c == Card::J { o } else { *c })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    HandType::from(&cards_wildcard)
                })
                .max()
                .unwrap()
        } else {
            hand_pt1.hand_type
        };

        HandPt2 {
            cards: hand_pt1
                .cards
                .into_iter()
                .map(CardPt2::from_pt1)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            hand_type,
        }
    }
}

impl Ord for HandPt2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards[0].cmp(&other.cards[0]))
            .then_with(|| self.cards[1].cmp(&other.cards[1]))
            .then_with(|| self.cards[2].cmp(&other.cards[2]))
            .then_with(|| self.cards[3].cmp(&other.cards[3]))
            .then_with(|| self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for HandPt2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardPt2 {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl CardPt2 {
    fn from_pt1(card: Card) -> CardPt2 {
        match card {
            Card::N2 => CardPt2::N2,
            Card::N3 => CardPt2::N3,
            Card::N4 => CardPt2::N4,
            Card::N5 => CardPt2::N5,
            Card::N6 => CardPt2::N6,
            Card::N7 => CardPt2::N7,
            Card::N8 => CardPt2::N8,
            Card::N9 => CardPt2::N9,
            Card::T => CardPt2::T,
            Card::J => CardPt2::J,
            Card::Q => CardPt2::Q,
            Card::K => CardPt2::K,
            Card::A => CardPt2::A,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1() {
        let d = Day07 {};
        assert_eq!(d.star1(INPUT), "6440");
    }

    #[test]
    fn part2() {
        let d = Day07 {};
        assert_eq!(d.star2(INPUT), "5905");
    }

    #[test]
    fn hand_types() {
        assert_eq!(Hand::parse("AAAAA").hand_type, HandType::Five);
        assert_eq!(Hand::parse("AA8AA").hand_type, HandType::Four);
        assert_eq!(Hand::parse("23332").hand_type, HandType::FullHouse);
        assert_eq!(Hand::parse("TTT98").hand_type, HandType::Three);
        assert_eq!(Hand::parse("23432").hand_type, HandType::TwoPair);
        assert_eq!(Hand::parse("A23A4").hand_type, HandType::OnePair);
        assert_eq!(Hand::parse("23456").hand_type, HandType::High);
    }

    #[test]
    fn hands_cmp() {
        let hand1 = Hand::parse("33332");
        let hand2 = Hand::parse("2AAAA");

        assert_eq!(hand1.hand_type, HandType::Four);
        assert_eq!(hand2.hand_type, HandType::Four);
        assert!(hand1 > hand2);

        let hand1 = Hand::parse("77888");
        let hand2 = Hand::parse("77788");

        assert_eq!(hand1.hand_type, HandType::FullHouse);
        assert_eq!(hand2.hand_type, HandType::FullHouse);
        assert!(hand1 > hand2);
    }
}
