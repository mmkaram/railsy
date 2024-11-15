use serde_json::json;
use serde_json::Value;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        match get_domains().await {
        Ok(domains) => {
            println!("Available domains:");
            for domain in domains {
                println!("- {}", domain);
            }
        },
        Err(e) => println!("Error fetching domains: {}", e),
    }

    Ok(())
}

async fn create_account(email: &str, password: &str) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let response = client.post("https://api.mail.tm/accounts")
        .header("Content-Type", "application/json")
        .json(&json!({
            "address": email,
            "password": password
        }))
        .send()
        .await?;
    if status.is_success() {
        serde_json::from_str(&body).map_err(|e| Error::new(status, e.to_string()))
    } else {
        Err(Error::new(status, format!("Account creation failed: Status: {}, Body: {}", status, body)))
}


async fn make_account(domains: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Choose the first domain (you could implement a different selection method)
    let domain = &domains[0];

    // Generate a random username (you might want to use a more sophisticated method)
    let username = format!("user{}", rand::random::<u32>());
    let email = format!("{}@{}", username, domain);
    let password = "YourStrongPassword123!"; // Consider generating a random password

    // Create the account
    match create_account(&email, password).await {
        Ok(account_data) => println!("Account created successfully: {:?}", account_data),
        Err(e) => println!("Failed to create account: {}", e),
    }

    Ok(())
}
async fn get_domains() -> Result<Vec<String>, Error> {
    let client = reqwest::Client::new();
    let response = client.get("https://api.mail.tm/domains")
        .send()
        .await?;

    let body: Value = response.json().await?;
    
    // Extract domains from the response
    let domains: Vec<String> = body["hydra:member"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|domain| domain["domain"].as_str().map(String::from))
        .collect();

    Ok(domains)
}

