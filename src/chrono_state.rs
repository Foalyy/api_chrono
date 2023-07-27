use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono_rs::TimeZone;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

use crate::utils::{timestamp, Timestamp};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LightColor {
    #[default]
    Unknown,
    Red,
    Orange,
    Green,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct SafetyChecks {
    pub weather: LightColor,
    pub zone_safety: LightColor,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct Zones {
    pub mf: LightColor,
    pub fx: LightColor,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct NextLaunch {
    #[serde(skip_deserializing)]
    pub last_update: Timestamp,
    #[serde(flatten)]
    pub project: Project,
    #[serde(flatten)]
    pub launchpad: Launchpad,
    pub countdown: isize,
    pub countdown_paused: bool,
}

pub type ProjectCode = String;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct Project {
    pub id: Option<usize>,
    pub code: ProjectCode,
    pub name: String,
    pub club_name: String,
    #[serde(default)]
    pub rocket_color: String,
    #[serde(default)]
    pub parachute_color: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct TrackedProject {
    pub arrival_time: Timestamp,
    #[serde(flatten)]
    pub project: Project,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct Launchpad {
    pub launchpad_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct LaunchpadDetails {
    pub id: String,
    pub launchpad_type: LaunchpadType,
    pub launchpad_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub enum LaunchpadType {
    #[default]
    FX,
    MF,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct LaunchpadState {
    pub last_update: Timestamp,
    pub countdown: isize,
    pub countdown_paused: bool,
    pub waiting_time: usize,
    #[serde(flatten)]
    pub project: TrackedProject,
    pub special_project: bool,
    pub upper_ailerons: bool,
    pub two_stage: bool,
    pub roll_control: bool,
    pub supersonic: bool,
    pub rf: String,
    pub rf_onboard: bool,
    pub rf_check: bool,
    pub rf_on: bool,
    pub rf_frequency: String,
    pub launchpad_set: bool,
    pub motor_type: String,
    pub motor_destocked: bool,
    pub motor_inserted: bool,
    pub motor_wired: bool,
    pub pyro_ok: bool,
    pub club_ok: bool,
    pub ddo_ok: bool,
    pub rso_ok: bool,
    pub landed: bool,
    pub flight_result: FlightResult,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum FlightResult {
    #[default]
    Unknown,
    Nominal,
    Balistic,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct LaunchpadsFX {
    pub plaintcontrix: Option<LaunchpadState>,
    pub toutatis: Option<LaunchpadState>,
    pub menhir: Option<LaunchpadState>,
    pub obelix: Option<LaunchpadState>,
}

#[derive(Debug, Clone, Default, Serialize, Hash)]
pub struct LaunchpadsMF {
    pub falballa: Option<LaunchpadState>,
    pub grossebaf: Option<LaunchpadState>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ChronoState {
    pub last_update: Option<Timestamp>,
    pub zones: Zones,
    pub safety_checks: SafetyChecks,
    pub next_launch: Option<NextLaunch>,
    pub clubs_tent: Vec<TrackedProject>,
    pub jupiter: Vec<TrackedProject>,
    pub launchpads_fx: LaunchpadsFX,
    pub launchpads_mf: LaunchpadsMF,
    pub message: String,

    #[serde(skip)]
    pub last_update_formated: Option<String>,
    #[serde(skip)]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SafetyChecksUpdate {
    pub weather: Option<LightColor>,
    pub zone_safety: Option<LightColor>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ZonesUpdate {
    pub mf: Option<LightColor>,
    pub fx: Option<LightColor>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LaunchpadsFXUpdate {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub plaintcontrix: Option<Option<LaunchpadStateUpdate>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub toutatis: Option<Option<LaunchpadStateUpdate>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub menhir: Option<Option<LaunchpadStateUpdate>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub obelix: Option<Option<LaunchpadStateUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LaunchpadsMFUpdate {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub falballa: Option<Option<LaunchpadStateUpdate>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub grossebaf: Option<Option<LaunchpadStateUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LaunchpadStateUpdate {
    pub countdown: Option<isize>,
    pub countdown_paused: Option<bool>,
    pub waiting_time: Option<usize>,
    #[serde(flatten)]
    pub project: Option<TrackedProject>,
    pub special_project: Option<bool>,
    pub upper_ailerons: Option<bool>,
    pub two_stage: Option<bool>,
    pub roll_control: Option<bool>,
    pub supersonic: Option<bool>,
    pub rf: Option<String>,
    pub rf_onboard: Option<bool>,
    pub rf_check: Option<bool>,
    pub rf_on: Option<bool>,
    pub rf_frequency: Option<String>,
    pub launchpad_set: Option<bool>,
    pub motor_type: Option<String>,
    pub motor_destocked: Option<bool>,
    pub motor_inserted: Option<bool>,
    pub motor_wired: Option<bool>,
    pub pyro_ok: Option<bool>,
    pub club_ok: Option<bool>,
    pub ddo_ok: Option<bool>,
    pub rso_ok: Option<bool>,
    pub landed: Option<bool>,
    pub flight_result: Option<FlightResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChronoStateUpdate {
    pub zones: Option<ZonesUpdate>,
    pub safety_checks: Option<SafetyChecksUpdate>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub next_launch: Option<Option<NextLaunch>>,
    pub clubs_tent: Option<Vec<TrackedProject>>,
    pub jupiter: Option<Vec<TrackedProject>>,
    pub launchpads_fx: Option<LaunchpadsFXUpdate>,
    pub launchpads_mf: Option<LaunchpadsMFUpdate>,
    pub message: Option<String>,
}

impl ChronoState {
    pub fn reset(&mut self) {
        std::mem::take(self);
    }

    pub fn update_with(&mut self, update: ChronoStateUpdate) {
        let current_timestamp = timestamp();
        self.last_update = Some(current_timestamp);
        if let Some(zones) = update.zones {
            if let Some(mf) = zones.mf {
                self.zones.mf = mf;
            }
            if let Some(fx) = zones.fx {
                self.zones.fx = fx;
            }
        }
        if let Some(safety_checks) = update.safety_checks {
            if let Some(weather) = safety_checks.weather {
                self.safety_checks.weather = weather;
            }
            if let Some(zone_safety) = safety_checks.zone_safety {
                self.safety_checks.zone_safety = zone_safety;
            }
        }
        if let Some(next_launch) = update.next_launch {
            self.next_launch = next_launch;
            if let Some(next_launch) = &mut self.next_launch {
                next_launch.last_update = current_timestamp;
            }
        }
        if let Some(clubs_tent) = update.clubs_tent {
            self.clubs_tent = clubs_tent;
        }
        if let Some(jupiter) = update.jupiter {
            self.jupiter = jupiter;
        }
        if let Some(launchpads_fx) = update.launchpads_fx {
            if let Some(plaintcontrix) = launchpads_fx.plaintcontrix {
                Self::update_launchpad_state_with(
                    &mut self.launchpads_fx.plaintcontrix,
                    plaintcontrix,
                );
            }
            if let Some(toutatis) = launchpads_fx.toutatis {
                Self::update_launchpad_state_with(&mut self.launchpads_fx.toutatis, toutatis);
            }
            if let Some(menhir) = launchpads_fx.menhir {
                Self::update_launchpad_state_with(&mut self.launchpads_fx.menhir, menhir);
            }
            if let Some(obelix) = launchpads_fx.obelix {
                Self::update_launchpad_state_with(&mut self.launchpads_fx.obelix, obelix);
            }
        }
        if let Some(launchpads_mf) = update.launchpads_mf {
            if let Some(falballa) = launchpads_mf.falballa {
                Self::update_launchpad_state_with(&mut self.launchpads_mf.falballa, falballa);
            }
            if let Some(grossebaf) = launchpads_mf.grossebaf {
                Self::update_launchpad_state_with(&mut self.launchpads_mf.grossebaf, grossebaf);
            }
        }
        if let Some(message) = update.message {
            self.message = message;
        }

        if let Some(last_update) = self.last_update {
            if let chrono_rs::LocalResult::Single(last_modified) =
                chrono_rs::Utc.timestamp_opt(last_update as i64, 0)
            {
                self.last_update_formated = Some(format!(
                    "{}",
                    last_modified.format("%a, %d %b %Y %H:%M:%S GMT")
                ));
            }
        }
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        self.hash = Some(format!("{}", hasher.finish()));
    }

    fn update_launchpad_state_with(
        state: &mut Option<LaunchpadState>,
        update: Option<LaunchpadStateUpdate>,
    ) {
        if let Some(update) = update {
            if state.is_none() {
                *state = Some(LaunchpadState::default());
            }
            if let Some(state) = state {
                state.last_update = timestamp();
                if let Some(countdown) = update.countdown {
                    state.countdown = countdown;
                }
                if let Some(countdown_paused) = update.countdown_paused {
                    state.countdown_paused = countdown_paused;
                }
                if let Some(waiting_time) = update.waiting_time {
                    state.waiting_time = waiting_time;
                }
                if let Some(project) = update.project {
                    state.project = project;
                }
                if let Some(special_project) = update.special_project {
                    state.special_project = special_project;
                }
                if let Some(upper_ailerons) = update.upper_ailerons {
                    state.upper_ailerons = upper_ailerons;
                }
                if let Some(two_stage) = update.two_stage {
                    state.two_stage = two_stage;
                }
                if let Some(roll_control) = update.roll_control {
                    state.roll_control = roll_control;
                }
                if let Some(supersonic) = update.supersonic {
                    state.supersonic = supersonic;
                }
                if let Some(rf) = update.rf {
                    state.rf = rf;
                }
                if let Some(rf_onboard) = update.rf_onboard {
                    state.rf_onboard = rf_onboard;
                }
                if let Some(rf_check) = update.rf_check {
                    state.rf_check = rf_check;
                }
                if let Some(rf_on) = update.rf_on {
                    state.rf_on = rf_on;
                }
                if let Some(rf_frequency) = update.rf_frequency {
                    state.rf_frequency = rf_frequency;
                }
                if let Some(launchpad_set) = update.launchpad_set {
                    state.launchpad_set = launchpad_set;
                }
                if let Some(motor_type) = update.motor_type {
                    state.motor_type = motor_type;
                }
                if let Some(motor_destocked) = update.motor_destocked {
                    state.motor_destocked = motor_destocked;
                }
                if let Some(motor_inserted) = update.motor_inserted {
                    state.motor_inserted = motor_inserted;
                }
                if let Some(motor_wired) = update.motor_wired {
                    state.motor_wired = motor_wired;
                }
                if let Some(pyro_ok) = update.pyro_ok {
                    state.pyro_ok = pyro_ok;
                }
                if let Some(club_ok) = update.club_ok {
                    state.club_ok = club_ok;
                }
                if let Some(ddo_ok) = update.ddo_ok {
                    state.ddo_ok = ddo_ok;
                }
                if let Some(rso_ok) = update.rso_ok {
                    state.rso_ok = rso_ok;
                }
                if let Some(landed) = update.landed {
                    state.landed = landed;
                }
                if let Some(flight_result) = update.flight_result {
                    state.flight_result = flight_result;
                }
            }
        } else {
            *state = None;
        }
    }
}

impl Hash for ChronoState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.last_update.hash(state);
        self.zones.hash(state);
        self.safety_checks.hash(state);
        self.next_launch.hash(state);
        self.clubs_tent.hash(state);
        self.jupiter.hash(state);
        self.launchpads_fx.hash(state);
        self.launchpads_mf.hash(state);
        self.message.hash(state);
    }
}

impl<'r> Responder<'r, 'static> for ChronoState {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let mut response = Response::build_from(Json(&self).respond_to(request)?);
        if let Some(last_update_formated) = self.last_update_formated {
            response.raw_header(
                rocket::http::hyper::header::LAST_MODIFIED.as_str(),
                last_update_formated,
            );
        }
        if let Some(hash) = self.hash {
            response.raw_header(rocket::http::hyper::header::ETAG.as_str(), hash);
        }
        response.ok()
    }
}
