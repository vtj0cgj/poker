use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                cards.push(Card { rank, suit });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
