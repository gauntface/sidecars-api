extern crate jsonwebtoken;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use jsonwebtoken::{encode, Algorithm, Header};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub metadata: Option<String>,
    pub contents: Option<String>,
    pub issues: Option<String>,
    pub single_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubApp {
    pub id: u32,
    pub node_id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub html_url: String,
    pub permissions: Permissions,
    pub events: Vec<String>,
}

pub fn get_app() -> Result<GitHubApp, Box<dyn std::error::Error>> {
    // Set your GitHub App's App ID and Private Key
    let app_id =
        env::var("GITHUB_APP_ID").expect("Environment variable GITHUB_APP_ID is not defined.");
    let private_key = env::var("GITHUB_APP_PRIVATE_KEY")
        .expect("Environment variable GITHUB_APP_PRIVATE_KEY is not defined.");

    // Generate a JSON Web Token (JWT) for GitHub App authentication
    let token = encode(
        &Header::new(Algorithm::RS256),
        &json!({
                "iat": chrono::Utc::now().timestamp(),
                "exp": chrono::Utc::now().timestamp() + 600, // Set expiration time (10 minutes)
                "iss": app_id,
        }),
        &jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())?,
    )?;

    // Create headers with the JWT for GitHub App authentication
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("User-Agent", HeaderValue::from_str("sidecars")?);
    headers.insert(
        "Accept",
        HeaderValue::from_str("application/vnd.github+json")?,
    );

    // Send a POST request to the GitHub GraphQL API
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/app")
        .headers(headers)
        .send()?;

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response JSON
        let github_app: GitHubApp = response.json()?;
        return Ok(github_app);
    }

    Err(Box::from(format!(
        "GitHub App request failed: ({}) {:?}",
        response.status(),
        response.text()
    )))
}

pub fn outgoing_prs() -> Result<(), Box<dyn std::error::Error>> {
    // GraphQL Query
    let _query = r#"
    {
        user(login: "YOUR_GITHUB_USERNAME") {
            login
            name
            email
        }
    }
    "#;

    // Set your GitHub App's App ID and Private Key
    let app_id =
        env::var("GITHUB_APP_ID").expect("Environment variable GITHUB_APP_ID is not defined.");
    let private_key = env::var("GITHUB_APP_PRIVATE_KEY")
        .expect("Environment variable GITHUB_APP_PRIVATE_KEY is not defined.");

    // Generate a JSON Web Token (JWT) for GitHub App authentication
    let token = encode(
        &Header::new(Algorithm::RS256),
        &json!({
                "iat": chrono::Utc::now().timestamp(),
                "exp": chrono::Utc::now().timestamp() + 600, // Set expiration time (10 minutes)
                "iss": app_id,
        }),
        &jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())?,
    )?;

    // Create headers with the JWT for GitHub App authentication
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("User-Agent", HeaderValue::from_str("sidecars")?);
    headers.insert(
        "Accept",
        HeaderValue::from_str("application/vnd.github+json")?,
    );

    /* let client = reqwest::blocking::Client::new();
    let install_response = client
        .get("https://api.github.com/app/installations")
        .headers(headers)
        .send()?;
    println!("Status:---------------> {}", install_response.status());
    println!("Body:-----------------> {}", install_response.text()?);*/

    let query = r#"
    {
        user(login: "gauntface") {
            login
            name
        }
    }
    "#;

    let mut request_body = std::collections::HashMap::new();
    request_body.insert("query", query);
    let json_body = serde_json::to_string(&request_body)?;

    // Send a POST request to the GitHub GraphQL API
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.github.com/graphql")
        .headers(headers)
        .body(json_body)
        .send()?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text()?);

    Ok(())
}
