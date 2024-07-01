use aws_config::BehaviorVersion;
use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use lstc_apigw::{parse_path, render_response, ApiError, Config, CorsHeaders, Outcome, Repository};
use lstc_domain::*;

pub struct Params {
    pub year: i32,
}

impl Params {
    pub fn from_path_parameters(params: &QueryMap) -> Result<Self, ApiError> {
        Ok(Self {
            year: parse_path(params, "year")?,
        })
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let cors = CorsHeaders::load_from_env().unwrap_or(CorsHeaders::empty());
    let outcome = match execute(event).await {
        Err(err) => Outcome::Error(err),
        Ok(item) => Outcome::Ok(item),
    };
    let response = render_response(outcome, &cors);
    Ok(response)
}

async fn execute(event: Request) -> Result<Vec<Event>, ApiError> {
    let params = Params::from_path_parameters(&event.path_parameters())?;
    let config = Config::load_from_env().unwrap();

    let sdk = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let repo = Repository::<Event>::new(&sdk, config.table_name);
    let pk = format!("E#{}", params.year);

    repo.query(pk).await
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
