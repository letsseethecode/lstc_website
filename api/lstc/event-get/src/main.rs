use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lstc_apigw::*;
use lstc_domain::*;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let params = event.path_parameters();
    let year = params.first("year").unwrap().parse::<i32>().unwrap();
    let month = params.first("month").unwrap().parse::<i32>().unwrap();
    let day = params.first("day").unwrap().parse::<i32>().unwrap();

    let headers = std::env::var("Access_Control_Allow_Headers").unwrap();
    let methods = std::env::var("Access_Control_Allow_Methods").unwrap();
    let origin = std::env::var("Access_Control_Allow_Origin").unwrap();
    let table_name = std::env::var("TABLE_NAME").unwrap();

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let client = Client::new(&config);
    let response = client
        .get_item()
        .table_name(table_name)
        .key(
            "pk".to_string(),
            AttributeValue::S(format!("E#{:04}", year)),
        )
        .key(
            "sk".to_string(),
            AttributeValue::S(format!("{:04}-{:02}-{:02}", year, month, day)),
        )
        .send()
        .await
        .unwrap();

    let resp = if let Some(item) = response.item {
        let body = Envelope::<Event> {
            message: "Ok".to_string(),
            warnings: vec![Warning::new("This endpoint is mocked")],
            faults: vec![],
            data: Event::new(
                item.get("sk").unwrap().as_s().unwrap().to_string(),
                item.get("title").unwrap().as_s().unwrap().to_string(),
                item.get("sub_title").unwrap().as_s().unwrap().to_string(),
                item.get("body").unwrap().as_s().unwrap().to_string(),
            ),
        };
        Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Headers", headers)
            .header("Access-Control-Allow-Methods", methods)
            .header("Access-Control-Allow-Origin", origin)
            .body(serde_json::to_string(&body).unwrap().into())
            .map_err(Box::new)?
    } else {
        let body = Envelope::<()> {
            message: "Ok".to_string(),
            warnings: vec![Warning::new("This endpoint is mocked")],
            faults: vec![],
            data: (),
        };
        Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Headers", headers)
            .header("Access-Control-Allow-Methods", methods)
            .header("Access-Control-Allow-Origin", origin)
            .body(serde_json::to_string(&body).unwrap().into())
            .map_err(Box::new)?
    };

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
