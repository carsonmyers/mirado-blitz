use std::collections::HashMap;

use itertools::Itertools;

use crate::error::Error;
use crate::models::{Card, Deck, Hand, Player};

pub enum GameState {
    Waiting,
    Started { turn: usize, knocked: Option<usize> },
    RoundEnded { eliminated: Vec<Player> },
    GameEnded { winner: Player },
}

pub struct Game {
    players: Vec<Player>,
    player_map: HashMap<String, usize>,
    deck: Deck,
    discard: Deck,
    round: usize,
    state: GameState,
}

impl Game {
    pub fn new(player: Player) -> Self {
        let mut deck = Deck::new_52();
        deck.shuffle();

        let mut player_map = HashMap::new();
        player_map.insert(player.id.clone(), 0);

        let players = vec![player];

        let discard = Deck::new();
        let round = 0;
        let state = GameState::Waiting;

        Game {
            players,
            player_map,
            deck,
            discard,
            round,
            state,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn player(&self, id: &str) -> Option<&Player> {
        self.player_map.get(id).map(|index| &self.players[*index])
    }

    pub fn join(&mut self, player: Player) -> Result<(), Error> {
        let GameState::Waiting = self.state else {
            return Err(Error::GameAlreadyStarted)
        };

        if self.players.len() >= 6 {
            Err(Error::MaxPlayersReached)
        } else {
            self.player_map
                .insert(player.id.clone(), self.players.len());
            self.players.push(player);

            Ok(())
        }
    }

    pub fn leave(&mut self, player_index: usize) -> Result<(), Error> {
        let GameState::Waiting = self.state else {
            return Err(Error::GameAlreadyStarted)
        };

        if player_index >= self.players.len() {
            Err(Error::InvalidPlayerIndex(player_index))
        } else {
            self.players.remove(player_index);
            self.player_map.clear();
            for (i, player) in self.players.iter().enumerate() {
                self.player_map.insert(player.id.clone(), i);
            }

            Ok(())
        }
    }

    pub fn start_round(&mut self) -> Result<(), Error> {
        match self.state {
            GameState::Waiting => {
                for player in self.players.iter_mut() {
                    player.lives = 4;
                    player.hand = Some(Hand::deal(&mut self.deck)?)
                }

                self.discard.push(self.deck.pop().ok_or(Error::DeckEmpty)?);

                self.state = GameState::Started {
                    turn: 0,
                    knocked: None,
                };
                Ok(())
            }
            GameState::RoundEnded { .. } => {
                self.deck = Deck::new_52();
                self.deck.shuffle();

                self.discard = Deck::new();

                for player in self.players.iter_mut() {
                    if player.lives == 0 {
                        player.hand = None;
                    } else {
                        player.hand = Some(Hand::deal(&mut self.deck)?);
                    }
                }

                self.discard.push(self.deck.pop().ok_or(Error::DeckEmpty)?);

                self.state = GameState::Started {
                    turn: 0,
                    knocked: None,
                };

                Ok(())
            }
            _ => Err(Error::GameAlreadyStarted),
        }
    }

    pub fn get_discard(&self) -> Result<Card, Error> {
        let card = self.discard.top().ok_or(Error::DeckEmpty)?.clone();
        Ok(card)
    }

    pub fn play_turn(&mut self, turn: Turn) -> Result<(), Error> {
        match self.state {
            GameState::Started {
                turn: player_turn,
                knocked,
            } => {
                if player_turn >= self.players.len() {
                    return Err(Error::InvalidPlayerIndex(player_turn));
                }

                let next_knocked_state = match turn {
                    Turn::Draw { index } => {
                        if self.deck.len() == 0 {
                            self.reshuffle()?;
                        }

                        let hand = self.players[player_turn].hand.as_mut().unwrap();
                        let discard = hand.replace(index, self.deck.pop().unwrap())?;

                        self.discard.push(discard);

                        if hand.value() == 31 {
                            return self.end_round();
                        }

                        knocked
                    }
                    Turn::DrawDiscard { index } => {
                        let hand = self.players[player_turn].hand.as_mut().unwrap();
                        let discard = hand.replace(index, self.discard.pop().unwrap())?;

                        self.discard.push(discard);

                        if hand.value() == 31 {
                            return self.end_round();
                        }

                        knocked
                    }
                    Turn::Knock => {
                        if let Some(knocked) = knocked {
                            return Err(Error::PlayerAlreadyKnocked(knocked));
                        }

                        Some(player_turn)
                    }
                };

                let next_turn = self.next_turn();
                if let Some(knocked) = next_knocked_state {
                    if next_turn == knocked {
                        return self.end_round();
                    }
                };

                self.state = GameState::Started {
                    turn: next_turn,
                    knocked: next_knocked_state,
                };
                Ok(())
            }
            GameState::RoundEnded { .. } => Err(Error::RoundNotStarted),
            _ => Err(Error::GameNotStarted),
        }
    }

    pub fn reset(&mut self) {
        self.deck = Deck::new_52();
        self.deck.shuffle();

        self.discard = Deck::new();

        for player in self.players.iter_mut() {
            player.lives = 4;
            player.hand = None;
        }

        self.state = GameState::Waiting;
    }

    fn end_round(&mut self) -> Result<(), Error> {
        let GameState::Started { knocked, .. } = self.state else {
            return Err(Error::RoundNotStarted);
        };

        let mut eliminations = Vec::new();

        let is_blitz = self.is_blitz();
        let lowest_score = self.lowest_score();

        let players = self
            .players
            .iter_mut()
            .enumerate()
            .filter(|(_, player)| player.hand.is_some());

        for (i, player) in players {
            let player_score = player.hand.as_ref().unwrap().value();

            // filter out non-losing players
            if is_blitz && player_score == 31 || !is_blitz && player_score > lowest_score {
                continue;
            }

            // a losing player loses twice as bad if they knocked
            if knocked
                .map(|player_index| player_index == i)
                .unwrap_or_default()
            {
                player.lives -= 2;
            } else {
                player.lives -= 1;
            }

            // eliminate any players with no lives left
            if player.lives <= 0 {
                player.lives = 0;
                eliminations.push(i);
            }
        }

        let mut remaining = self
            .players
            .iter()
            .filter(|player| player.lives > 0)
            .collect_vec();

        if remaining.len() == 1 {
            self.state = GameState::GameEnded {
                winner: remaining.pop().unwrap().clone(),
            }
        } else if remaining.len() > 1 {
            self.state = GameState::RoundEnded {
                eliminated: eliminations
                    .into_iter()
                    .map(|i| self.players.get(i).unwrap().clone())
                    .collect_vec(),
            }
        } else {
            return Err(Error::NoMorePlayers);
        }

        Ok(())
    }

    fn reshuffle(&mut self) -> Result<(), Error> {
        self.deck.add(&mut self.discard);
        self.deck.shuffle();
        self.discard.push(self.deck.pop().ok_or(Error::DeckEmpty)?);

        Ok(())
    }

    fn is_blitz(&self) -> bool {
        self.players.iter().any(|player| {
            player
                .hand
                .as_ref()
                .map(|hand| hand.value() == 31)
                .unwrap_or_default()
        })
    }

    fn lowest_score(&self) -> u32 {
        self.players
            .iter()
            .filter_map(|player| player.hand.as_ref().map(|hand| hand.value()))
            .min()
            .unwrap_or_default()
    }

    fn next_turn(&self) -> usize {
        match self.state {
            GameState::Started { turn, .. } => {
                let mut next_turn = self.adv_turn(turn);
                while self.players[next_turn].lives == 0 {
                    next_turn = self.adv_turn(next_turn);
                }

                next_turn
            }
            _ => 0,
        }
    }

    fn adv_turn(&self, turn: usize) -> usize {
        (turn + 1) % self.players.len()
    }
}

pub enum Turn {
    Draw { index: usize },
    DrawDiscard { index: usize },
    Knock,
}
