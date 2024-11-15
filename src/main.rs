use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Domain {
    domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    subject: String,
    intro: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let base_url = "https://api.mail.tm";

    println!("Hello?");
    // Get available domains
    let domains: Vec<Domain> = client
        .get(format!("{}/domains", base_url))
        .send()
        .await?
        .json()
        .await?;

    let domain = &domains[0].domain;
    println!("Using domain: {}", domain);

    // Generate a random email address
    let random_username = format!("user{}", rand::random::<u32>());
    let email = format!("{}@{}", random_username, domain);
    let password = "your_secure_password";

    println!("Generated email: {}", email);

    // Create an account
    let account: Account = client
        .post(format!("{}/accounts", base_url))
        .json(&serde_json::json!({
            "address": email,
            "password": password
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("Account created: {:?}", account);

    // Get authentication token
    let token: Token = client
        .post(format!("{}/token", base_url))
        .json(&serde_json::json!({
            "address": email,
            "password": password
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("Token received: {}", token.token);

    // Set up authenticated client
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token.token))?,
    );
    let auth_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Check for messages
    let messages: Vec<Message> = auth_client
        .get(format!("{}/messages", base_url))
        .send()
        .await?
        .json()
        .await?;

    println!("Number of messages: {}", messages.len());

    for message in messages {
        println!("Message: {:?}", message);
    }

    // Delete the account (optional)
    let delete_response = auth_client
        .delete(format!("{}/accounts/{}", base_url, account.id))
        .send()
        .await?;

    if delete_response.status().is_success() {
        println!("Account deleted successfully");
    } else {
        println!("Failed to delete account");
    }

    Ok(())
}
