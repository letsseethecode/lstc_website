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

    let headers = std::env::var("Access_Control_Allow_Headers").unwrap();
    let methods = std::env::var("Access_Control_Allow_Methods").unwrap();
    let origin = std::env::var("Access_Control_Allow_Origin").unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Headers", headers)
        .header("Access-Control-Allow-Methods", methods)
        .header("Access-Control-Allow-Origin", origin)
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
