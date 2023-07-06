use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::serde::Deserialize;
use rocket::State;

#[derive(Deserialize)]
pub struct Config {
    api_key: String,
}

pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingHeader,
    InvalidKey,
    InvalidConfig,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.guard::<&State<Config>>().await {
            request::Outcome::Success(config) => {
                if !config.api_key.is_empty() {
                    match request.headers().get_one("api_key") {
                        Some(provided_key) => {
                            if provided_key == config.api_key {
                                request::Outcome::Success(ApiKey(provided_key.to_string()))
                            } else {
                                request::Outcome::Failure((
                                    Status::Forbidden,
                                    ApiKeyError::InvalidKey,
                                ))
                            }
                        }
                        None => request::Outcome::Failure((
                            Status::BadRequest,
                            ApiKeyError::MissingHeader,
                        )),
                    }
                } else {
                    request::Outcome::Failure((
                        Status::InternalServerError,
                        ApiKeyError::InvalidConfig,
                    ))
                }
            }
            _ => {
                request::Outcome::Failure((Status::InternalServerError, ApiKeyError::InvalidConfig))
            }
        }
    }
}
