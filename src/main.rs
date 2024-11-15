use anyhow::Result;
use railsy::TempEmailClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = TempEmailClient::new();

    // Generate a new email
    let email = client.generate_email().await?;
    println!("Generated email: {}", email);

    // Check inbox
    println!("Checking inbox...");
    let messages = client.check_inbox().await?;
    
    if messages.is_empty() {
        println!("No messages in the inbox.");
    } else {
        for message in messages {
            println!("From: {} <{}>", message.from.name, message.from.address);
            println!("Subject: {}", message.subject);
            println!("Intro: {}", message.intro);
            println!("---");
        }
    }

    Ok(())
}
