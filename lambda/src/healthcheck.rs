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

async fn handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
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
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn test_handler_with_environment() {
        env::set_var("FOCUSED_ENV", "example");

        let context = Context::default();
        let payload = Request {};
        let event = LambdaEvent { payload, context };
        let result = handler(event).await.unwrap();
        assert_eq!(result.statusCode, 200);
        assert_eq!(result.body.status, "ok");
        assert_eq!(result.body.environment, "example");
    }
}
