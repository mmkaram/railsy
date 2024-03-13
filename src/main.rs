use std::fs::File;
use std::io::Write;
use serde_json::{Value, to_string_pretty};
use ureq::get;

fn main() {
    // Make the API request
    let response = get("https://api.mail.tm/accounts")
        .call();

    // Check if the request was successful
    match response {
        Ok(res) => {
            // Check if the response status is OK
            if res.status() == 200 {
                // Parse the JSON response
                let json_body: Value = serde_json::from_str(&res.into_string().unwrap()).unwrap();

                // Write JSON to a file
                let mut file = File::create("accounts.json").expect("Unable to create file");
                file.write_all(to_string_pretty(&json_body).unwrap().as_bytes())
                    .expect("Unable to write data to file");

                println!("JSON response saved to accounts.json");
            } else {
                println!("Error: {}", res.status());
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

