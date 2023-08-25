use strum::EnumIter;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: CardSuit,
    pub face: CardFace,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuit {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardFace {
    Ace,
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
}

impl Card {
    pub fn new(suit: CardSuit, face: CardFace) -> Self {
        Card { suit, face }
    }

    pub fn blitz_value(&self) -> u32 {
        match self.face {
            CardFace::Ace => 11,
            CardFace::Two => 2,
            CardFace::Three => 3,
            CardFace::Four => 4,
            CardFace::Five => 5,
            CardFace::Six => 6,
            CardFace::Seven => 7,
            CardFace::Eight => 8,
            CardFace::Nine => 9,
            CardFace::Ten => 10,
            CardFace::Jack => 10,
            CardFace::Queen => 10,
            CardFace::King => 10,
        }
    }
}
