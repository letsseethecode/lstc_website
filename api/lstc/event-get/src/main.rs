use aws_config::BehaviorVersion;
use lambda_http::{
    aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use lstc_apigw::{render_response, Config, CorsHeaders, Outcome, Repository};
use lstc_domain::*;

pub struct Params {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Params {
    pub fn from_path_parameters(params: &QueryMap) -> Self {
        Self {
            year: params.first("year").unwrap().parse::<i32>().unwrap(),
            month: params.first("month").unwrap().parse::<i32>().unwrap(),
            day: params.first("day").unwrap().parse::<i32>().unwrap(),
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
    let sk = format!("{:04}-{:02}-{:02}", params.year, params.month, params.day);
    let outcome = if let Some(item) = repo.load(pk, sk).await {
        Outcome::Ok(item)
    } else {
        Outcome::NotFound
    };
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
