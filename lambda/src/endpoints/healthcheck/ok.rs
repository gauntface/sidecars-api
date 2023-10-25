use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Serialize;
use std::env;

use sidecars::apis::github;

#[derive(Serialize)]
struct ResponseBody {
    status: String,
    environment: String,
    app: String,
}

#[derive(Serialize)]
struct ErrorResponseBody {
    message: String,
}

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let app = github::get_app();
    if app.is_err() {
        let body = ErrorResponseBody {
            message: format!("Failed to get GitHub App: {}", app.err().unwrap()),
        };
        let resp = Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&body).unwrap().into())
            .unwrap();

        return Ok(resp);
    }

    let body = ResponseBody {
        status: "ok".to_string(),
        environment: env::var("SIDECARS_ENV").unwrap_or("unknown".to_string()),
        app: app.unwrap().name,
    };
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&body).unwrap().into())
        .unwrap();

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http;

    #[tokio::test]
    async fn test_handler_with_environment() {
        env::set_var("SIDECARS_ENV", "test");
        env::set_var("GITHUB_APP_ID", "1234");
        env::set_var(
            "GITHUB_APP_PRIVATE_KEY",
            "----- EXAMPLE -----\nTesting\n----- END -----\n",
        );

        let req = http::request::Builder::new()
            .method(http::method::Method::GET)
            .body(Body::Empty)
            .unwrap();
        let result = handler(req).await.unwrap();

        assert_eq!(result.status(), 500);
        let want = ErrorResponseBody {
            message: format!("Failed to get GitHub App: {}", "InvalidKeyFormat"),
        };
        assert_eq!(
            result.into_body(),
            serde_json::to_string(&want).unwrap().into()
        );
    }
}
