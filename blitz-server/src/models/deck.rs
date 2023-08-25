use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::models::{Card, CardFace, CardSuit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new_52() -> Deck {
        let mut deck = Deck::new();

        for suit in CardSuit::iter() {
            for face in CardFace::iter() {
                deck.push(Card::new(suit, face));
            }
        }

        deck
    }

    pub fn new() -> Deck {
        Deck { cards: Vec::new() }
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn add(&mut self, other: &mut Deck) {
        self.cards.append(&mut other.cards);
    }

    pub fn top(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Iterator for Deck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
