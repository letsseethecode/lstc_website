use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lstc_apigw::*;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let params = event.path_parameters();
    let year = params.first("year").unwrap();

    let headers = std::env::var("Access_Control_Allow_Headers").unwrap();
    let methods = std::env::var("Access_Control_Allow_Methods").unwrap();
    let origin = std::env::var("Access_Control_Allow_Origin").unwrap();
    let table_name = std::env::var("TABLE_NAME").unwrap();

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let client = Client::new(&config);
    let results = client
        .query()
        .table_name(table_name)
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_values(":pk", AttributeValue::S(format!("E#{}", year)))
        .send()
        .await
        .unwrap();

    let body = if let Some(items) = results.items {
        Envelope::<Vec<Event>> {
            message: "Ok".to_string(),
            warnings: vec![Warning::new("This endpoint is mocked")],
            faults: vec![],
            data: items
                .iter()
                .map(|item| {
                    Event::new(
                        item.get("sk").unwrap().as_s().unwrap().to_string(),
                        item.get("header").unwrap().as_s().unwrap().to_string(),
                        "".to_string(),
                    )
                })
                .collect(),
        }
    } else {
        Envelope::<Vec<Event>> {
            message: "Ok".to_string(),
            warnings: vec![Warning::new("This endpoint is mocked")],
            faults: vec![],
            data: vec![],
        }
    };

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

#[cfg(test)]
mod test {
    use aws_config::BehaviorVersion;
    use aws_sdk_dynamodb::{types::AttributeValue, Client};

    #[tokio::test]
    async fn do_it_do_it_now() {
        let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .load()
            .await;
        let client = Client::new(&config);
        let results = client
            .query()
            .table_name("lstc_website--www--data")
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(":pk", AttributeValue::S("E#2024".to_string()))
            .send()
            .await
            .unwrap();

        if let Some(items) = results.items {
            println!("Results");
            for item in items {
                println!(
                    "pk={:?}, sk={:?}, header={:?}",
                    item.get("pk").unwrap(),
                    item.get("sk").unwrap(),
                    item.get("header").unwrap()
                );
                println!("{:?}", item);
            }
        } else {
            println!("No results!!");
        }
    }
}
