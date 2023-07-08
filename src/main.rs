use std::ops::Deref;

use chrono_state::{ChronoState, ChronoStateUpdate, LaunchpadDetails, LaunchpadType};
use config::Config;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, tokio::sync::RwLock, State};
use rocket_dyn_templates::{context, Template};
use utils::timestamp;

use crate::config::ApiKey;
use crate::utils::Timestamp;

#[macro_use]
extern crate rocket;

mod chrono_state;
mod config;
mod utils;

#[get("/")]
fn index(launchpads: &State<Vec<LaunchpadDetails>>) -> Template {
    Template::render(
        "main",
        context! {
            time_url: uri!(time()).to_string(),
            state_url: uri!(state()).to_string(),
            launchpads: launchpads.deref()
        },
    )
}

#[get("/time")]
async fn time() -> Json<Timestamp> {
    Json(timestamp())
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
    let launchpads = vec![
        LaunchpadDetails {
            id: "plaintcontrix".to_string(),
            launchpad_type: LaunchpadType::FX,
            launchpad_name: "Plaintcontrix".to_string(),
        },
        LaunchpadDetails {
            id: "toutatis".to_string(),
            launchpad_type: LaunchpadType::FX,
            launchpad_name: "Toutatis".to_string(),
        },
        LaunchpadDetails {
            id: "menhir".to_string(),
            launchpad_type: LaunchpadType::FX,
            launchpad_name: "Menhir".to_string(),
        },
        LaunchpadDetails {
            id: "obelix".to_string(),
            launchpad_type: LaunchpadType::FX,
            launchpad_name: "Obelix".to_string(),
        },
        LaunchpadDetails {
            id: "falballa".to_string(),
            launchpad_type: LaunchpadType::MF,
            launchpad_name: "Falballa".to_string(),
        },
        LaunchpadDetails {
            id: "grossebaf".to_string(),
            launchpad_type: LaunchpadType::MF,
            launchpad_name: "Grossebaf".to_string(),
        },
    ];

    rocket::build()
        .mount("/", routes![index, time, state, update_state, reset_state])
        .mount("/static", FileServer::from("static/").rank(0))
        .manage(RwLock::new(ChronoState::default()))
        .manage(launchpads)
        .attach(AdHoc::config::<Config>())
        .attach(Template::fairing())
}
