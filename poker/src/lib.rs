use std::cmp::Ordering;
use std::iter::zip;

#[derive(Debug)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    fn from(c: char) -> Self {
        match c {
            'S' => Suit::Spades,
            'C' => Suit::Clubs,
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Card(u8, Suit);

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Debug)]
pub struct Hand<'a> {
    cards: Vec<Card>,
    original: &'a str,
}

pub fn get_rank(group: &str) -> u32 {
    let mut chars = group.chars();
    match chars.next() {
        Some('A') => 14,
        Some('K') => 13,
        Some('Q') => 12,
        Some('J') => 11,
        Some(_) =>
            group.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0),
        None => 0
    }
}

impl<'a> PartialEq<Self> for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.cards.len() {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        return true
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
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
    }
}

pub enum PokerHand {
    HighCard,
    TwoOfAKind,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    Straight,
    Flush,
    FourOfAKind,
    RoyalFlush,
}

fn get_hand(hand: &str) -> Hand {
    let mut cards: Vec<Card> = hand.split_ascii_whitespace()
        .map(|group| {
            Card(
                get_rank(group) as u8,
                Suit::from(group.chars().last().unwrap_or('_'))
            )
        })
        .collect();
    cards.sort_by(|r, l| l.0.partial_cmp(&r.0).unwrap());
    Hand { cards, original: hand }
}

fn compare_hands<'a>(lhs: Hand<'a>, rhs: Hand<'a>) -> Hand<'a> {
    lhs
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
