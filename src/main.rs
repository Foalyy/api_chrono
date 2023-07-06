use chrono_state::{ChronoState, ChronoStateUpdate};
use config::Config;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, tokio::sync::RwLock, State};

use crate::config::ApiKey;

#[macro_use]
extern crate rocket;

mod chrono_state;
mod config;
mod utils;

#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/state")]
async fn state(chrono_state: &State<RwLock<ChronoState>>) -> Json<ChronoState> {
    Json(chrono_state.read().await.clone())
}

#[post("/state", format = "application/json", data = "<update>")]
async fn update_state(
    _api_key: ApiKey,
    update: Json<ChronoStateUpdate>,
    chrono_state: &State<RwLock<ChronoState>>,
) -> Json<ChronoState> {
    let mut state = chrono_state.write().await;
    state.update_with(update.into_inner());
    Json(state.clone())
}

#[post("/reset")]
async fn reset_state(_api_key: ApiKey, chrono_state: &State<RwLock<ChronoState>>) {
    let mut state = chrono_state.write().await;
    state.reset()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, state, update_state, reset_state])
        .manage(RwLock::new(ChronoState::default()))
        .attach(AdHoc::config::<Config>())
}
