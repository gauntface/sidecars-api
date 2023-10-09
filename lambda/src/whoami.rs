use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

async fn handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    let resp = Response {
        statusCode: 200,
        body: "Hello Matt!".to_string(),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}
