use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn get_request_with_proxy() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("socks5://localhost:9050")?)
        .build()?;
    let request_builder = client.get("https://www.spotify.com/api/account-settings/v1/profile");
    let token = get_token_from_file("token.txt");
    let response = request_builder.header("Cookie", token).send().await?;

    println!("{}", response.text().await?);
    Ok(())
}

fn get_token_from_file(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().expect("Failed to read line");
    println!("{}", line);
    line
}