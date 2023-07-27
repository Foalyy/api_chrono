use std::ops::Deref;

use chrono_state::{ChronoState, ChronoStateUpdate, LaunchpadDetails, LaunchpadType};
use config::Config;
use rocket::fs::FileServer;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{fairing::AdHoc, tokio::sync::RwLock, Shutdown, State};
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
            state_stream_url: uri!(state_stream()).to_string(),
            launchpads: launchpads.deref()
        },
    )
}

#[get("/time")]
async fn time() -> Json<Timestamp> {
    Json(timestamp())
}

#[get("/state")]
async fn state(chrono_state: &State<RwLock<ChronoState>>) -> ChronoState {
    chrono_state.read().await.clone()
}

#[get("/state-stream")]
async fn state_stream(queue: &State<Sender<ChronoState>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let state = select! {
                state = rx.recv() => match state {
                    Ok(state) => state,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&state);
        }
    }
}

#[post("/state", format = "application/json", data = "<update>")]
async fn update_state(
    _api_key: ApiKey,
    update: Json<ChronoStateUpdate>,
    chrono_state: &State<RwLock<ChronoState>>,
    queue: &State<Sender<ChronoState>>,
) -> Json<ChronoState> {
    let mut state = chrono_state.write().await;
    state.update_with(update.into_inner());
    queue.send(state.clone()).ok();
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
        .mount(
            "/",
            routes![index, time, state, state_stream, update_state, reset_state],
        )
        .mount("/static", FileServer::from("static/").rank(0))
        .manage(RwLock::new(ChronoState::default()))
        .manage(channel::<ChronoState>(1).0)
        .manage(launchpads)
        .attach(AdHoc::config::<Config>())
        .attach(Template::fairing())
}
