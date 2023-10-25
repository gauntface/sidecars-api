use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Serialize;

use sidecars::apis::github;

#[derive(Serialize)]
struct ResponseBody {

}

#[derive(Serialize)]
struct ErrorResponseBody {
    message: String,
}

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let app = github::outgoing_prs();
    if app.is_err() {
        let body = ErrorResponseBody {
            message: format!("Failed to get outgoing PRs: {}", app.err().unwrap()),
        };
        let resp = Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&body).unwrap().into())
            .unwrap();

        return Ok(resp);
    }

    let body = ResponseBody {};
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
    use std::env;
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
            message: format!("Failed to get outgoing PRs: {}", "InvalidKeyFormat"),
        };
        assert_eq!(
            result.into_body(),
            serde_json::to_string(&want).unwrap().into()
        );
    }
}
