#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("deck is empty")]
    DeckEmpty,
    #[error("not enough cards in deck")]
    NotEnoughCardsInDeck,
    #[error("invalid hand index: {0}")]
    InvalidHandIndex(usize),
    #[error("game already started")]
    GameAlreadyStarted,
    #[error("game has not been started")]
    GameNotStarted,
    #[error("round has not been started")]
    RoundNotStarted,
    #[error("maxiumum number of players reached")]
    MaxPlayersReached,
    #[error("invalid player: {0}")]
    InvalidPlayerIndex(usize),
    #[error("player {0} has already knocked")]
    PlayerAlreadyKnocked(usize),
    #[error("no more players left")]
    NoMorePlayers,
    #[error("no hands have been dealt")]
    NoHands,
}
