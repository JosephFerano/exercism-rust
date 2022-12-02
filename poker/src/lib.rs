use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum HandType {
    HighCard,
    TwoOfAKind(u8),
    TwoPairs(u8, u8),
    ThreeOfAKind(u8),
    Straight,
    Flush,
    FullHouse(u8, u8),
    FourOfAKind(u8),
    StraightFlush,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum CardType {
    LowAce,
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl CardType {
    fn get_numeric_value(self) -> u8 {
        match self  {
            CardType::LowAce => 1,
            CardType::Num(n) => n,
            CardType::Jack => 11,
            CardType::Queen => 12,
            CardType::King => 13,
            CardType::Ace => 14,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    card_type: CardType,
    suit: char,
}

#[derive(Debug)]
pub struct Hand<'a> {
    cards: Vec<Card>,
    hand_type: HandType,
    original: &'a str,
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_type == other.card_type
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.card_type.partial_cmp(&other.card_type)
    }
}

impl<'a> PartialEq<Self> for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter()
            .zip(&other.cards)
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {
                for i in 0..self.cards.len() {
                    if self.cards[i] == other.cards[i] {
                        continue
                    } else if self.cards[i] < other.cards[i] {
                        return Some(Ordering::Less)
                    } else {
                        return Some(Ordering::Greater)
                    }
                }
                None
            }
            ord => ord,
        }
    }
}

pub fn get_rank(group: &str) -> CardType {
    let mut chars = group.chars();
    match chars.next() {
        Some('A') => CardType::Ace,
        Some('K') => CardType::King,
        Some('Q') => CardType::Queen,
        Some('J') => CardType::Jack,
        _ => {
            CardType::Num(group.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0))
        }
    }
}

fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let first = *cards.iter().peekable().peek().unwrap();
    let same_suit = cards.iter().all(|c| c.suit == first.suit);
    let is_straight = cards.windows(2).all(|cs| {
        let lhs = cs[0].card_type.get_numeric_value();
        let rhs = cs[1].card_type.get_numeric_value();
        (lhs - rhs) == 1
    });
    let (first_pair, second_pair) =
        cards.windows(2)
            .fold(((0, 1), None), |(fst_pair, snd_pair), elem| {
                let lhs = elem[0].card_type.get_numeric_value();
                let rhs = elem[1].card_type.get_numeric_value();
                if lhs == rhs {
                    if let Some((_, snd)) = snd_pair {
                        (fst_pair, Some((lhs, snd + 1)))
                    } else {
                        ((lhs, fst_pair.1 + 1), snd_pair)
                    }
                } else {
                    // Here we switch from incrementing the first pair to the second pair
                    if fst_pair.1 > 1 && snd_pair.is_none() {
                        (fst_pair, Some((0, 1)))
                    } else {
                        (fst_pair, snd_pair)
                    }
                }
            });
    match (same_suit, is_straight, first_pair, second_pair) {
        (true, true, _, _)             => HandType::StraightFlush,
        (_, _, (v,4), _)               => HandType::FourOfAKind(v),
        (_, _, (v1, 3), Some((v2, 2))) => HandType::FullHouse(v1, v2),
        (_, _, (v1, 2), Some((v2, 3))) => HandType::FullHouse(v2, v1),
        (true, _, _, _)                => HandType::Flush,
        (_, true, _, _)                => HandType::Straight,
        (_, _, (v, 3), _)              => HandType::ThreeOfAKind(v),
        (_, _, (v1, 2), Some((v2, 2))) => HandType::TwoPairs(v1, v2),
        (_, _, (v, 2), _)              => HandType::TwoOfAKind(v),
        (_, _, _, _)                   => HandType::HighCard,
    }
}

fn get_hand(original: &str) -> Hand {
    use crate::CardType::*;
    let mut cards: Vec<Card> = original.split_ascii_whitespace()
        .map(|group| {
            Card {
                card_type: get_rank(group),
                suit: group.chars().last().unwrap_or('_'),
            }
        })
        .collect();
    cards.sort_by(|r, l| l.card_type.partial_cmp(&r.card_type).unwrap());
    // Check for A2345 straight exception, we have to do this now since we sort here
    if cards[0].card_type == Ace
        && cards[1].card_type == Num(5)
        && cards[2].card_type == Num(4)
        && cards[3].card_type == Num(3)
        && cards[4].card_type == Num(2) {
        cards[0].card_type = LowAce;
        cards.sort_by(|r, l| l.card_type.partial_cmp(&r.card_type).unwrap());
    }
    let hand_type = get_hand_type(&cards);
    Hand { cards, hand_type, original }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    match hands {
        [_] => hands.to_vec(),
        hs => {
            let mut hands: Vec<Hand> = hs.iter().map(|str| get_hand(&str)).collect();
            hands.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            for i in (0..hands.len()).rev() {
                if i > 0 && hands[i] != hands[i-1] {
                    hands.remove(i);
                }
            }
            hands.iter().map(|h| h.original).collect()
        }
    }
}
