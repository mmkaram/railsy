use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

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
    let client = Client::new();
    let base_url = "https://api.mail.tm";

    // Get available domains
    let response = client.get(format!("{}/domains", base_url)).send().await?;
    let body: Value = response.json().await?;
    
    let domains: Vec<String> = body["hydra:member"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|domain| domain["domain"].as_str().map(String::from))
        .collect();

    let domain = &domains[0];
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
    let auth_client = Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token.token).parse().unwrap(),
            );
            headers
        })
        .build()?;

    // Check for messages
    let response = auth_client
        .get(format!("{}/messages", base_url))
        .send()
        .await?;

    let messages_body: Value = response.json().await?;
    let messages: Vec<Message> = messages_body["hydra:member"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|msg| serde_json::from_value(msg.clone()).ok())
        .collect();

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
