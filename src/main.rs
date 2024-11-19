use std::error::Error;

use clap::Parser;
use reqwest::Client;
use serde_json::Value; // For parsing JSON

#[derive(Parser)]
#[command(name = "hxr", version, author, about)]
#[command(about = "A simple HTTP request CLI tool", long_about = None)]
struct Cli {
    /// The HTTP method to use (GET, POST, PUT, DELETE, etc.)
    #[arg(short, long, default_value = "GET")]
    method: String,

    /// The URL to send the request to
    url: String,

    /// Optional JSON body for POST/PUT requests
    #[arg(short, long)]
    body: Option<String>,

    /// Optional headers in key:value format (can be repeated)
    #[arg(short, long, value_parser = parse_key_val, number_of_values = 1)]
    headers: Vec<(String, String)>,
}

// Helper function to parse key:value header pairs
pub fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid header format: '{}'. Use key:value.", s));
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let client = Client::new();
    let method = match cli.method.to_uppercase().as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => return Err("Invalid HTTP method".into()),
    };

    let mut request = client.request(method, &cli.url);

    for (key, value) in cli.headers {
        request = request.header(key, value);
    }

    if let Some(body) = cli.body {
        request = request.body(body);
    }

    let response = request.send().await?;

    // println!("Response Status: {}", response.status());

    let headers = response.headers().clone();
    let content_type = headers
        .get("content-type")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    let bytes = response.bytes().await?;
    let body = String::from_utf8_lossy(&bytes);

    if content_type.contains("application/json") {
        match serde_json::from_slice::<Value>(&bytes) {
            Ok(json) => println!("{}", serde_json::to_string_pretty(&json)?),
            Err(_) => println!("Invalid JSON response:\n{}", body),
        }
    } else {
        println!("Response Body:\n{}", body);
    }

    Ok(())
}
