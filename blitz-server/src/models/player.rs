use crate::models::Hand;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Option<Hand>,
    pub lives: u8,
}
