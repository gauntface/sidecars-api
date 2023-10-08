use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct ResponseBody {
    status: String,
    environment: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: ResponseBody,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    let resp = Response {
        statusCode: 200,
        body: ResponseBody {
            status: "ok".to_string(),
            environment: env::var("FOCUSED_ENV").unwrap_or("unknown".to_string()),
        },
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
