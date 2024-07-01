mod cors;
mod repository;

pub use cors::*;
use lstc_domain::Envelope;
pub use repository::*;

use lambda_http::{Body, Response};
use serde::Serialize;

pub struct Config {
    pub table_name: String,
}

impl Config {
    pub fn load_from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            table_name: std::env::var("TABLE_NAME")?,
        })
    }
}

pub enum Outcome<T> {
    Ok(T),
    NotFound,
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
    };
    builder.map_err(Box::new).unwrap()
}
