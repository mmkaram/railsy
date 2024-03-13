use reqwest::{blocking, header::{CONTENT_LENGTH, COOKIE}};
use serde::Deserialize;
use std::collections::hash_map;

fn main () {
    blocking_get().unwrap();
}

fn blocking_get() -> Result<(), std::error::Error> {
    // let res = reqwest::blocking::post("https://api.mail.tm/accounts");
    let res = reqwest::blocking::get("https://google.com");

    let body = res.text()?;
    println!("body = {}", body);
    
    Ok(())
}