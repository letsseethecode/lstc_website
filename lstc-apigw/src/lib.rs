mod cors;
mod repository;

use std::str::FromStr;

pub use cors::*;
use lstc_domain::Envelope;
pub use repository::*;

use lambda_http::{aws_lambda_events::query_map::QueryMap, Body, Response};
use serde::Serialize;

pub struct Config {
    pub table_name: String,
}

impl Config {
    pub fn load_from_env() -> Result<Self, ApiError> {
        Ok(Self {
            table_name: std::env::var("TABLE_NAME").map_err(|_| ApiError::BadEnv("TABLE_NAME"))?,
        })
    }
}

pub fn parse_path<T: FromStr>(params: &QueryMap, name: &'static str) -> Result<T, ApiError> {
    params
        .first(name)
        .ok_or(ApiError::MissingPath(name))
        .map(|v| v.parse::<T>().map_err(|_| ApiError::BadPath(name)))?
}

#[derive(Debug, Serialize)]
pub enum ApiError {
    MissingPath(&'static str),
    BadPath(&'static str),
    BadEnv(&'static str),
    DatabaseError,
}

pub enum Outcome<T> {
    Ok(T),
    NotFound,
    Error(ApiError),
}

pub fn render_response<T: Serialize>(outcome: Outcome<T>, cors: &CorsHeaders) -> Response<Body> {
    let builder = Response::builder()
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Headers", &cors.headers)
        .header("Access-Control-Allow-Methods", &cors.methods)
        .header("Access-Control-Allow-Origin", &cors.origin);
    let builder = match outcome {
        Outcome::Ok(data) => {
            let body = Envelope {
                message: "OK".to_string(),
                warnings: vec![],
                faults: vec![],
                data,
            };
            builder
                .status(200)
                .body(serde_json::to_string(&body).unwrap().into())
        }
        Outcome::NotFound => {
            let body = Envelope {
                message: "Not Found".to_string(),
                warnings: vec![],
                faults: vec![],
                data: (),
            };
            builder
                .status(404)
                .body(serde_json::to_string(&body).unwrap().into())
        }
        Outcome::Error(error) => {
            let body = Envelope {
                message: "Internal Server Error".to_string(),
                warnings: vec![],
                faults: vec![],
                data: error,
            };
            builder
                .status(500)
                .body(serde_json::to_string(&body).unwrap().into())
        }
    };
    builder.map_err(Box::new).unwrap()
}
