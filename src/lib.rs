use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

const API_BASE_URL: &str = "https://api.mail.tm";

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: String,
    address: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: EmailAddress,
    pub subject: String,
    pub intro: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    pub address: String,
    pub name: String,
}

pub struct TempEmailClient {
    client: reqwest::Client,
    account: Option<Account>,
}

impl TempEmailClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            account: None,
        }
    }

    pub async fn generate_email(&mut self) -> Result<String> {
        let domains = self.get_domains().await?;
        let domain = domains.first().ok_or_else(|| anyhow!("No domains available"))?;

        let password = "temporaryPassword123!";
        let address = format!("user{}@{}", rand::random::<u32>(), domain);

        let account = self
            .client
            .post(format!("{}/accounts", API_BASE_URL))
            .json(&json!({
                "address": address,
                "password": password
            }))
            .send()
            .await?
            .json::<Account>()
            .await?;

        let token = self.get_token(&address, password).await?;
        self.account = Some(Account {
            token,
            ..account
        });

        Ok(address)
    }


    pub async fn check_inbox(&self) -> Result<Vec<Message>> {
    let account = self.account.as_ref().ok_or_else(|| anyhow!("No account generated"))?;

    let response = self
        .client
        .get(format!("{}/messages", API_BASE_URL))
        .header("Authorization", format!("Bearer {}", account.token))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Check if the response is an object with a "hydra:member" key
    if let Some(messages) = response.get("hydra:member") {
        let messages: Vec<Message> = serde_json::from_value(messages.clone())?;
        Ok(messages)
    } else {
        // If not, try to parse the entire response as a Vec<Message>
        let messages: Vec<Message> = serde_json::from_value(response)?;
        Ok(messages)
    }
}
    async fn get_domains(&self) -> Result<Vec<String>> {
        let domains = self
            .client
            .get(format!("{}/domains", API_BASE_URL))
            .send()
            .await?
            .json::<Vec<serde_json::Value>>()
            .await?;

        Ok(domains
            .into_iter()
            .filter_map(|domain| domain["domain"].as_str().map(String::from))
            .collect())
    }

    async fn get_token(&self, address: &str, password: &str) -> Result<String> {
        let response = self
            .client
            .post(format!("{}/token", API_BASE_URL))
            .json(&json!({
                "address": address,
                "password": password
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        response["token"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| anyhow!("Failed to get token"))
    }
}
