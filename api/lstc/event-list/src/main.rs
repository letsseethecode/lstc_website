use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use lstc_apigw::{render_response, Config, CorsHeaders, Outcome, Repository};
use lstc_domain::*;

pub struct Params {
    pub year: i32,
}

impl Params {
    pub fn from_path_parameters(params: &QueryMap) -> Self {
        Self {
            year: params.first("year").unwrap().parse::<i32>().unwrap(),
        }
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let params = Params::from_path_parameters(&event.path_parameters());
    let cors = CorsHeaders::load_from_env().unwrap();
    let config = Config::load_from_env().unwrap();

    let sdk = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let repo = Repository::<Event>::new(&sdk, config.table_name);
    let pk = format!("E#{}", params.year);

    let items = repo.query(pk).await;
    let outcome = Outcome::Ok(items);

    let response = render_response(outcome, &cors);
    Ok(response)
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
