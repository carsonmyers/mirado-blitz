#[macro_use]
extern crate rocket;

mod error;
mod models;
mod routes;
mod state;

#[launch]
fn rocket() -> _ {
    rocket::build().manage(state::GamesState::new())
}
