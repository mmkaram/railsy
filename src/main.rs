use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    post().await?;

    Ok(())
}

async fn post() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.mail.tm/accounts")
        .timeout(tokio::time::Duration::from_secs(5))
        .send()
        .await?;

    let body = res.text().await?;

    let data: Value = serde_json::from_str(&body)?;
    
    println!("body = {:?}", body);
    println!("json = {:?}", data);

    Ok(())
}

