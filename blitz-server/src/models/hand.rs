use itertools::Itertools;

use crate::error::Error;
use crate::models::{Card, Deck};

#[derive(Debug, Clone)]
pub struct Hand {
    cards: (Card, Card, Card),
}

impl Hand {
    pub fn deal(deck: &mut Deck) -> Result<Hand, Error> {
        let cards = deck
            .take(3)
            .collect_tuple()
            .ok_or(Error::NotEnoughCardsInDeck)?;

        Ok(Hand { cards })
    }

    pub fn replace(&mut self, index: usize, card: Card) -> Result<Card, Error> {
        match index {
            0 => Ok(std::mem::replace(&mut self.cards.0, card)),
            1 => Ok(std::mem::replace(&mut self.cards.1, card)),
            2 => Ok(std::mem::replace(&mut self.cards.2, card)),
            _ => Err(Error::InvalidHandIndex(index)),
        }
    }

    pub fn cards(&self) -> [&Card; 3] {
        [&self.cards.0, &self.cards.1, &self.cards.2]
    }

    pub fn value(&self) -> u32 {
        let suits = self
            .cards()
            .into_iter()
            .sorted_by(|a, b| a.suit.cmp(&b.suit))
            .group_by(|card| card.suit);

        suits
            .into_iter()
            .map(|(_, cards)| {
                cards
                    .into_iter()
                    .map(|card| card.blitz_value())
                    .sum::<u32>()
            })
            .max()
            .unwrap_or(0)
    }
}
