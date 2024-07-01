use std::str::FromStr;

use aws_config::BehaviorVersion;
use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use lstc_apigw::{parse_path, render_response, ApiError, Config, CorsHeaders, Outcome, Repository};
use lstc_domain::*;

pub struct Params {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Params {
    pub fn from_path_parameters(params: &QueryMap) -> Result<Self, ApiError> {
        Ok(Self {
            year: parse_path::<i32>(params, "year")?,
            month: parse_path::<i32>(params, "month")?,
            day: parse_path::<i32>(params, "day")?,
        })
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let cors = CorsHeaders::load_from_env().unwrap_or(CorsHeaders::empty());
    let outcome = match execute(event).await {
        Err(err) => Outcome::Error(err),
        Ok(Some(item)) => Outcome::Ok(item),
        Ok(None) => Outcome::NotFound,
    };
    let response = render_response(outcome, &cors);
    Ok(response)
}

async fn execute(event: Request) -> Result<Option<Event>, ApiError> {
    let params = Params::from_path_parameters(&event.path_parameters())?;
    let config = Config::load_from_env()?;
    let sdk = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let repo = Repository::<Event>::new(&sdk, config.table_name);
    let pk = format!("E#{}", params.year);
    let sk = format!("{:04}-{:02}-{:02}", params.year, params.month, params.day);
    repo.load(pk, sk).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
