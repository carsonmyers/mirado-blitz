use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::models::Game;

pub struct GamesState(Arc<RwLock<HashMap<String, Mutex<Game>>>>);

impl GamesState {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
}
