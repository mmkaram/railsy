use reqwest::{
    blocking,
    header::{CONTENT_LENGTH, COOKIE},
};
use serde::Deserialize;
use std::collections::hash_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    post().await?;

    Ok(())
}

fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("https://api.mail.tm/accounts");
    // let res = reqwest::blocking::get("https://google.com");

    let body = res?.text()?;
    println!("body = {}", body);

    Ok(())
}

async fn post() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.mail.tm/accounts")
        // .header(CONTENT_LENGTH, 27)
        .timeout(tokio::time::Duration::from_secs(5))
        // .body("the exact body that is sent")
        .send()
        .await?;

    let body = res.text().await?;
    println!("body = {:?}", body);

    Ok(())
}
