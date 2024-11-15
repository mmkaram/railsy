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

async fn post () -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.post("https://api.mail.tm/accounts")
        .header("Content-Type", "application/json")
        .json(&json!({
            "address": "87serious@goeschman.com",
            "password": "string"
        }))
        .send()
        .await?;

    println!("Response status: {}", response.status());
    println!("Response body: {}", response.text().await?);

    Ok(())
}
