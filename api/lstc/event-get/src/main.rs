use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use lstc_apigw::{
    event_pk, event_sk, parse_path, render_response, ApiError, Config, CorsHeaders, Outcome,
    Repository,
};
use lstc_domain::Event;

pub struct Params {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Params {
    pub fn from_path_parameters(params: &QueryMap) -> Result<Self, ApiError> {
        Ok(Self {
            year: parse_path::<i32>(params, "year")?,
            month: parse_path::<u32>(params, "month")?,
            day: parse_path::<u32>(params, "day")?,
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
    let repo = Repository::<Event>::new_from_config(config.table_name).await;
    repo.load(
        event_pk(params.year),
        event_sk(params.year, params.month, params.day),
    )
    .await
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
