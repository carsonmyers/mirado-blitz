use rocket::http::{Header, ContentType, CookieJar};
use rocket::serde::{json::Json, Deserialize};
use rocket::response::Responder;
use rocket::State;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPlayer<'req> {
    pub player_name: &'req str,
};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct NewGame {
    pub game_id: String,
    
};


#[post("/new_game", data = "<player>")]
pub fn new_game(player: Json<NewPlayer<'_>>, cookies: &CookieJar<'_>) {}
