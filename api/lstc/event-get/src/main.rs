use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lstc_apigw::*;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let params = event.path_parameters();
    let year = params.first("year").unwrap_or("??");
    let month = params.first("month").unwrap_or("??");
    let day = params.first("day").unwrap_or("??");

    let body = Envelope::<Event> {
        message: "OK".to_string(),
        warnings: vec![Warning::new("This endpoint is mocked")],
        faults: vec![],
        data: Event::new(
            format!("{year}-{month}-{day}"),
            "The first of January".to_string(),
            "# This is some markdown

            We've got a bunch of code here

            * And
            * A
            * Bulleted
            * List"
                .to_string(),
        ),
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
