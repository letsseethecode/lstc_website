use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lstc_apigw::*;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let body = Envelope::<Vec::<Event>> {
        message: "Ok".to_string(),
        warnings: vec![
            Warning::new("This endpoint is mocked"),
        ],
        faults: vec![],
        data: vec![
            Event::new("2024-01-01", "Headline", "Body"),
            Event::new("2024-01-02", "Headline", "Body"),
            Event::new("2024-01-03", "Headline", "Body"),
        ]
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&body).unwrap().into())
        .map_err(Box::new)?;

    Ok(resp)
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
