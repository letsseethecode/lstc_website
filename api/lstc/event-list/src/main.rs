use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lstc_apigw::*;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let body = Envelope::<Vec<Event>> {
        message: "Ok".to_string(),
        warnings: vec![Warning::new("This endpoint is mocked")],
        faults: vec![],
        data: vec![
            Event::new("2024-08-19", "August 2024 (in-person)", "Welcome"),
            Event::new("2024-07-15", "July 2024 (in-person)", "Body"),
            Event::new("2024-06-07", "June 2024 (in-person)", "Body"),
            Event::new("2024-05-20", "May 2024 (in-person)", "Body"),
            Event::new("2024-04-15", "April 2024 (in-person)", "Body"),
            Event::new("2024-03-18", "March 2024 (in-person)", "Body"),
        ],
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type,X-Amz-Date,Authorization,X-Api-Key,X-Amz-Security-Token",
        )
        .header("Access-Control-Allow-Methods", "GET,OPTIONS")
        .header("Access-Control-Allow-Origin", "*")
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
