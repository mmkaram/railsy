use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::io::{self, Write};

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

#[derive(Debug, Serialize, Deserialize)]
struct MessageDetail {
    #[serde(default)]
    id: String,
    #[serde(default)]
    subject: String,
    #[serde(default)]
    intro: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    html: Option<String>,
    #[serde(default)]
    from: EmailAddress,
    #[serde(default)]
    to: Vec<EmailAddress>,
    #[serde(default)]
    seen: bool,
    #[serde(default)]
    flagged: bool,
    #[serde(rename = "downloadUrl", default)]
    download_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct EmailAddress {
    #[serde(default)]
    address: String,
    #[serde(default)]
    name: Option<String>,
}

async fn create_email_account(
    client: &Client,
    base_url: &str,
) -> Result<(Account, String, Token), Box<dyn Error>> {
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

    Ok((account, email, token))
}

async fn get_messages(client: &Client, base_url: &str) -> Result<Vec<Message>, Box<dyn Error>> {
    let response = client.get(format!("{}/messages", base_url)).send().await?;
    let messages_body: Value = response.json().await?;
    let messages: Vec<Message> = messages_body["hydra:member"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|msg| serde_json::from_value(msg.clone()).ok())
        .collect();
    Ok(messages)
}

async fn get_message_by_id(
    client: &Client,
    base_url: &str,
    message_id: &str,
) -> Result<MessageDetail, Box<dyn Error>> {
    let response = client
        .get(format!("{}/messages/{}", base_url, message_id))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch message. Status: {}", response.status()).into());
    }

    let text = response.text().await?;
    match serde_json::from_str(&text) {
        Ok(message) => Ok(message),
        Err(e) => {
            println!("Debug - Response body: {}", text);
            Err(format!("Failed to parse message: {}", e).into())
        }
    }
}

async fn save_message_by_id(
    client: &Client,
    base_url: &str,
    message_id: &str,
    //) -> Result<String, Box<dyn Error>> {
) {
    _ = match get_message_by_id(&client, &base_url, &message_id).await {
        Ok(message) => Result::Ok(save_to_file(&message.text, "output.txt")),
        Err(e) => Result::Err(e),
    };
}

use std::fs::File;
use std::path::Path;

fn save_to_file(content: &str, dir: &str) -> std::io::Result<()> {
    // Construct the full path
    let file_path = Path::new(dir);

    // Create the file and write the content
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!("File saved to {}", file_path.display());
    Ok(())
}

async fn delete_message(
    client: &Client,
    base_url: &str,
    message_id: &str,
) -> Result<(), Box<dyn Error>> {
    let response = client
        .delete(format!("{}/messages/{}", base_url, message_id))
        .send()
        .await?;

    if response.status().is_success() {
        println!("Message deleted successfully");
        Ok(())
    } else {
        Err("Failed to delete message".into())
    }
}

fn print_menu() {
    println!("\nEmail Client Menu:");
    println!("1. View Messages");
    println!("2. Read Message (by ID)");
    println!("3. Delete Message");
    println!("4. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let base_url = "https://api.mail.tm";

    println!("Creating new email account...");
    let (account, email, token) = create_email_account(&client, base_url).await?;
    println!("Account created successfully!");
    println!("Your email address is: {}", email);

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

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let messages = get_messages(&auth_client, base_url).await?;
                println!("\nMessages ({}):", messages.len());
                for msg in messages {
                    println!("ID: {}", msg.id);
                    println!("Subject: {}", msg.subject);
                    println!("Preview: {}", msg.intro);
                    println!("---");
                }
            }
            "2" => {
                print!("Enter message ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;

                match get_message_by_id(&auth_client, base_url, id.trim()).await {
                    Ok(message) => {
                        println!(
                            "\nFrom: {} {}",
                            message.from.address,
                            message.from.name.unwrap_or_default()
                        );
                        println!("Subject: {}", message.subject);
                        println!("Content:\n{}", message.text);
                        if !message.seen {
                            println!("Status: Unread");
                        }
                        if let Some(url) = message.download_url {
                            println!("Download URL: {}", url);
                        }
                    }
                    Err(e) => println!("Error reading message: {}", e),
                }
            }
            "3" => {
                print!("Enter message ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;

                match delete_message(&auth_client, base_url, id.trim()).await {
                    Ok(_) => println!("Message deleted successfully"),
                    Err(e) => println!("Error deleting message: {}", e),
                }
            }
            "4" => {
                println!("Exiting...");
                // Delete the account before exiting
                let delete_response = auth_client
                    .delete(format!("{}/accounts/{}", base_url, account.id))
                    .send()
                    .await?;

                if delete_response.status().is_success() {
                    println!("Account deleted successfully");
                } else {
                    println!("Failed to delete account");
                }
                break;
            }
            "5" => {
                print!("Enter message ID to write to disk: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id)?;
                save_message_by_id(&client, base_url, &id).await;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }

    Ok(())
}
